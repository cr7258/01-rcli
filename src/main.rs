use clap::Parser;
use rcli::{CmdExecutor, Opts};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化 tracing_subscriber 的日志系统
    tracing_subscriber::fmt::init();
    let opts: Opts = Opts::parse();
    opts.cmd.execute().await?;
    Ok(())
}
