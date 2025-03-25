use kovi::tokio::sync::broadcast;
use kovi::MsgEvent;
use std::cell::RefCell;
use std::collections::HashSet;
use std::sync::Arc;


struct BotCommandBuilder {
    event_bus: broadcast::Sender<BotCommand>,
    super_command: RefCell<HashSet<&'static str>>,
    common_command: RefCell<HashSet<&'static str>>,
}
kovi::tokio::task_local! {
    static COMMAND_BUILDER:BotCommandBuilder;
}
impl BotCommandBuilder {
    pub async fn init_event_bus() {
        let (tx, _) = broadcast::channel(100);
        let b = BotCommandBuilder {
            event_bus: tx,
            super_command: RefCell::new(HashSet::new()),
            common_command: RefCell::new(HashSet::new()),
        };
        COMMAND_BUILDER.scope(b, async {}).await;
    }
    pub fn on_common_command<F, Fut>(cmd: &'static str, hd: F)
    where
        F: Fn(BotCommand) -> Fut + Send + Sync + 'static,
        Fut: Future + Send,
        Fut::Output: Send,
    {
        COMMAND_BUILDER.with(|f| {
            f.common_command.borrow_mut().insert(cmd);
            f.subscribe(cmd, hd);
        })
    }
    pub fn on_super_command<F, Fut>(cmd: &'static str, hd: F)
    where
        F: Fn(BotCommand) -> Fut + Send + Sync + 'static,
        Fut: Future + Send,
        Fut::Output: Send,
    {
        COMMAND_BUILDER.with(|f| {
            f.super_command.borrow_mut().insert(cmd);
            f.subscribe(cmd, hd);
        })
    }
    fn subscribe<F, Fut>(&self, cmd: &'static str, hd: F)
    where
        F: Fn(BotCommand) -> Fut + Send + Sync + 'static,
        Fut: Future + Send,
        Fut::Output: Send,
    {
        let mut rec = self.event_bus.subscribe();
        kovi::spawn(async move {
            while let Ok(event) = rec.recv().await {
                if (*event.cmd) != cmd {
                    continue;
                }
                hd(event).await;
            }
        });
    }
}
#[derive(Debug, Clone)]
pub struct BotCommand {
    cmd: Arc<String>,
    args: Arc<Vec<String>>,
    event: Arc<MsgEvent>,
}
impl BotCommand {
    pub fn from_str(s: &str, e: Arc<MsgEvent>) -> BotCommand {
        let mut args = s.split_whitespace();
        BotCommand {
            event: e,
            cmd: Arc::new(args.next().expect("怎么可能为空捏").to_string()),
            args: Arc::new(args.map(|x| x.to_string()).collect()),
        }
    }
    pub fn invoke_command(&self) {}
    fn exec_common_command(&self) {}
    fn exec_super_command(&self) {}
}
