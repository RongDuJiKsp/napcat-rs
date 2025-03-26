use crate::command_exec::app::{BotCommand, BotCommandBuilder};
use crate::config::SyncControl;
use anyhow::anyhow;
use boa_engine::error::JsErasedError;
use boa_engine::{JsResult, JsValue, Source};
use dashmap::mapref::one::Ref;
use dashmap::DashMap;
use kovi::chrono::{DateTime, NaiveDateTime, Utc};
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
use std::time::{SystemTime, UNIX_EPOCH};

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
            "help" => { e.event.reply_and_quote("$shell 指令目前支持Js Shell的喵。使用$shell new 创建新shell，$shell lock 锁定个人上下文shell $shell unlock 解锁个人上下文 $shell list shell和所有者列表") }
            "lock" => {}
            "unlock" => {}
            "list" => {}
            "new" => {}
            cmd => {
                if let Some(sh) = ShellMemory::get().shell(e.event.sender.user_id) {
                    e.event.reply_and_quote("异步任务创建成功喵！");
                    if let Err(_) = sh.js_eval_queue.send((Utc::now().timestamp(), cmd.to_string())).await {
                        e.event.reply_and_quote(format!("Js引擎已经停止运行了喵。原因：{}", sh.exit_error.read().await.as_ref().map(|x| x.as_str()).unwrap_or("none")))
                    }
                    match sh.res_queue.lock().await.recv().await {
                        None => {
                            e.event.reply_and_quote(format!("Js引擎已经停止运行了喵!原因：{}", sh.exit_error.read().await.as_ref().map(|x| x.as_str()).unwrap_or("none")));
                        }
                        Some((time, res)) => {
                            e.event.reply_and_quote(format!("任务{}的执行结果如下：{}", DateTime::from_timestamp(time, 0).unwrap().format("%Y-%m-%d %H:%M:%S"), match res {
                                Ok(e) => e.to_string(),
                                Err(e) => e.to_string()
                            }))
                        }
                    }
                } else {
                    e.event.reply_and_quote("你没有锁定自己的shell喵");
                }
            }
        }
    } else {
        e.event.reply_and_quote("请指定命令喵");
    }
}

#[derive(Debug, Default)]
struct ShellMemory {
    instance_gid: DashMap<usize, JavaScriptEngine>,
    id_gen: AtomicUsize,
    user_shell: DashMap<i64, usize>,
}
static SHELL_MEMORY: OnceLock<ShellMemory> = OnceLock::new();
impl ShellMemory {
    fn get() -> &'static ShellMemory {
        SHELL_MEMORY.get_or_init(|| ShellMemory::default())
    }
    fn new_shell(&self, owner: i64) -> Ref<usize, JavaScriptEngine> {
        let shell_id = self.id_gen.fetch_add(1, Ordering::Relaxed);
        self.instance_gid
            .entry(shell_id)
            .or_insert(JavaScriptEngine::new(owner))
            .downgrade()
    }
    fn shell(&self, uid: i64) -> Option<Ref<usize, JavaScriptEngine>> {
        self.user_shell
            .get(&uid)
            .and_then(|i| self.instance_gid.get(i.value()))
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
        let (resp_tx, resp_rs) =
            mpsc::channel::<(i64, Result<Value, JsErasedError>)>(JS_ENGINE_QUEUE_BUF_SIZE);
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
                    Err(e) => Err(e.into_erased(&mut ctx)),
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
