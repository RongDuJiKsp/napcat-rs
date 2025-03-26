use crate::command_exec::app::{BotCommand, BotCommandBuilder};
use crate::config::SyncControl;
use anyhow::anyhow;
use boa_engine::error::JsErasedError;
use boa_engine::{JsResult, JsValue, Source};
use dashmap::mapref::one::RefMut;
use dashmap::DashMap;
use kovi::log::{info, warn};
use kovi::serde_json::value::Value;
use kovi::tokio::sync::{mpsc, Mutex, RwLock};
use kovi::tokio::task::spawn_blocking;
use kovi::MsgEvent;
use std::collections::HashMap;
use std::error::Error;
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::sync::{Arc, OnceLock};
use std::thread::spawn;

async fn register_shell_cmd() {
    BotCommandBuilder::on_super_command("$shell", |e| exec_shell_cmd(e)).await;
}
async fn exec_shell_cmd(e: BotCommand) {
    if !SyncControl::running() {
        return;
    }
    let mut arg = e.args.iter();
    if let Some(sub_cmd) = arg.next() {
        match sub_cmd.as_str() {
            "help" => { e.event.reply_and_quote("$shell 指令目前支持Js Shell的喵。使用$shell new 创建新shell，$shell use 切换个人上下文shell") }
            "lock" => {}
            &_ => {}
        }
    } else {}

    e.event
        .reply_and_quote(format!("shell创建成功喵！编号 {}", shell_id));
}

#[derive(Debug, Default)]
struct ShellMemory {
    instance_gid: DashMap<usize, JavaScriptEngine>,
    id_gen: AtomicUsize,

}
static SHELL_MEMORY: OnceLock<ShellMemory> = OnceLock::new();
impl ShellMemory {
    fn get() -> &'static ShellMemory {
        SHELL_MEMORY.get_or_init(|| ShellMemory::default())
    }
    fn shell(&self, shell_id: usize) -> RefMut<usize, JavaScriptEngine> {
        self.instance_gid.entry(shell_id).or_insert(JavaScriptEngine::new())
    }
    fn new_shell(&self) -> RefMut<usize, JavaScriptEngine> {
        let shell_id = self.id_gen.fetch_add(1, Ordering::Relaxed);
        self.shell(shell_id)
    }
}
const JS_ENGINE_QUEUE_BUF_SIZE: usize = 50;
#[derive(Debug, Clone)]
struct JavaScriptEngine {
    owner: Arc<AtomicI64>,
    js_eval_queue: mpsc::Sender<(i64, String)>,
    res_queue: Arc<Mutex<mpsc::Receiver<(i64, Result<Value, JsErasedError>)>>>,
    exit_error: Arc<RwLock<Option<String>>>,
}
impl JavaScriptEngine {
    fn new(owner: i64) -> JavaScriptEngine {
        let (code_tx, mut code_rx) = mpsc::channel::<(i64, String)>(JS_ENGINE_QUEUE_BUF_SIZE);
        let (resp_tx, resp_rs) = mpsc::channel::<(i64, Result<Value, JsErasedError>)>(JS_ENGINE_QUEUE_BUF_SIZE);
        let err_output = Arc::new(RwLock::new(None));
        let err_input = err_output.clone();
        spawn(move || {
            let mut ctx = boa_engine::Context::default();
            let mut last_output = String::new();
            while let Some((submit_time, code)) = code_rx.blocking_recv() {
                let js_out = ctx.eval(Source::from_bytes(code.as_bytes()));
                last_output = format!("{:?}", &js_out);
                let rs_out: Result<Value, JsErasedError> = match js_out {
                    Ok(val) => {
                        if let JsValue::Undefined = &val {
                            Ok(Value::String(String::from("undefined")))
                        } else {
                            val.to_json(&mut ctx).map_err(|je| je.into_erased(&mut ctx))
                        }
                    }
                    Err(e) => { Err(e.into_erased(&mut ctx)) }
                };
                if let Err(_) = resp_tx.blocking_send((submit_time, rs_out)) {
                    *err_input.blocking_write() = Some(String::from("异常退出：输出写端已关闭"));
                    break;
                }
            }
            warn!(
                "JS引擎已退出:{}\n最后一次输出:{}",
              err_input
                    .blocking_read()
                    .as_ref()
                    .map(|x| x.as_str())
                    .unwrap_or("输入写端已退出"),
                last_output
            );
        });
        JavaScriptEngine {
            res_queue: Arc::new(Mutex::new(resp_rs)),
            js_eval_queue: code_tx,
            exit_error: err_output,
            owner: Arc::new(AtomicI64::new(owner)),
        }
    }
}
