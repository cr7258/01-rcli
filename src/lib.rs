mod cli;
mod process;
mod utils;

pub use cli::{
    Base64Format, Base64SubCommand, HttpSubCommand, Opts, SubCommand, TextSignFormat,
    TextSubCommand,
};
pub use process::*;
pub use utils::{get_content, get_reader};

// Rust 标准库尚未稳定地支持在 trait 中直接使用 async fn，
// 使用这个宏用于允许在 trait 中使用 async fn 而不产生编译器警告或错误。
#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
