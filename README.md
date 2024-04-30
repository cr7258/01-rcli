# 第一周：魔法神箭：从 Hello world 到实用的 CLI 工具

* [主要代码](#主要代码)
* [单元测试](#单元测试)
* [安装 rcli 工具到本地](#安装-rcli-工具到本地)

## 主要代码

通过定义 CmdExecutor trait，我们可以为不同的命令行工具提供统一的执行接口。

```rust
// Rust 标准库尚未稳定地支持在 trait 中直接使用 async fn，
// 使用这个宏用于允许在 trait 中使用 async fn 而不产生编译器警告或错误。
#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
```

例如 Base64EncodeOpts 和 Base64DecodeOpts 分别实现了 CmdExecutor trait，这样我们可以通过 Base64SubCommand 来统一执行 Base64EncodeOpts 和 Base64DecodeOpts。

```rust
impl CmdExecutor for Base64EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = crate::get_reader(&self.input)?;
        let ret = crate::process_encode(&mut reader, self.format)?;
        println!("{}", ret);
        Ok(())
    }
}

impl CmdExecutor for Base64DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = crate::get_reader(&self.input)?;
        let ret = crate::process_decode(&mut reader, self.format)?;
        println!("{}", ret);
        Ok(())
    }
}

impl CmdExecutor for Base64SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            Base64SubCommand::Encode(opts) => opts.execute().await,
            Base64SubCommand::Decode(opts) => opts.execute().await,
        }
    }
}
```

另外 Base64SubCommand 也实现了 CmdExecutor trait，这样我们可以通过 SubCommand 来统一执行 Base64SubCommand、TextSubCommand、HttpSubCommand 等。

```rust
impl CmdExecutor for SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            SubCommand::Csv(opts) => opts.execute().await,
            SubCommand::GenPass(opts) => opts.execute().await,
            SubCommand::Base64(cmd) => cmd.execute().await,
            SubCommand::Text(cmd) => cmd.execute().await,
            SubCommand::Http(cmd) => cmd.execute().await,
        }
    }
}
```

最终原先在 main.rs 复杂的命令行逻辑将会被精简为一句代码：

```rust
opts.cmd.execute().await?;
```

## 单元测试

```bash
cargo nextest run

Starting 7 tests across 2 binaries (run ID: 5f7c22fa-8461-4e0a-a734-bd72e5a2f233, nextest profile: default)
    PASS [   0.008s] rcli cli::tests::test_verify_file
    PASS [   0.008s] rcli process::b64::tests::test_process_decode
    PASS [   0.007s] rcli process::http_serve::tests::test_file_not_found
    PASS [   0.008s] rcli process::b64::tests::test_process_encode
    PASS [   0.008s] rcli process::http_serve::tests::test_file_handler
    PASS [   0.008s] rcli process::text::tests::test_blake3_sign_verify
    PASS [   0.008s] rcli process::text::tests::test_ed25519_sign_verify
------------
 Summary [   0.010s] 7 tests run: 7 passed, 0 skipped
```

## 安装 rcli 工具到本地

```bash
cargo install --path .
```

运行 rcli 工具测试功能：

生成密钥。

```bash
rcli genpass -l 20

# 输出
$PrfH4RTCn*3oJ^pi2%M
Password strength: 4
```

启动 HTTP 服务器。

```bash
rcli http serve
```
