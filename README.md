# 第一周：魔法神箭：从 Hello world 到实用的 CLI 工具

* [安装依赖](#安装依赖)
* [enum_dispatch 使用方法](#enum_dispatch-使用方法)
* [主要代码](#主要代码)
* [单元测试](#单元测试)
* [安装 rcli 工具到本地](#安装-rcli-工具到本地)

## 安装依赖

```bash
cargo add enum_dispatch
```

## enum_dispatch 使用方法

这段代码是一个简单的示例，展示了如何在 Rust 中使用 enum_dispatch 库来实现对枚举类型的动态分发。

```rust
use enum_dispatch::enum_dispatch;

// 首先，定义了一个名为 Animal 的 trait，该 trait 有一个方法 make_sound。
#[enum_dispatch]
trait Animal {
    fn make_sound(&self);
}

// 然后，定义了两个结构体 Dog 和 Cat，并为它们分别实现了 Animal trait。
// 这意味着 Dog 和 Cat 都有 make_sound 方法。
struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Bark!");
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
}

// 接着，定义了一个名为 Pet 的枚举，该枚举的每个变体都是 Animal trait 的实现者。
// 这是通过 #[enum_dispatch(Animal)] 宏实现的。
#[enum_dispatch(Animal)]
enum Pet {
    // 当你使用 enum_dispatch 宏时，你只需列出构成枚举的各个类型，而不需要显示地包装它们（像 Dog(Dog)） 这样。
    // enum_dispatch 宏在背后为你自动生成必要的代码来包装这些类型，并实现相应的 trait 调用。
    Dog,
    Cat,
}

// 最后，在 main 函数中，创建了一个 Pet 枚举的实例 my_pet，并调用了它的 make_sound 方法。
// 由于 my_pet 是 Dog 变体，所以输出的是 "Bark!"
fn main() {
    let my_pet: Pet = Pet::Dog(Dog);
    my_pet.make_sound();  // 输出 "Bark!"
}
```

如果不使用 enum_dispatch，我们需要为 Pet 枚举实现 Animal trait，然后在 match 表达式中根据 Pet 的变体调用相应的方法。

```rust
enum Pet {
    Dog(Dog),
    Cat(Cat),
}
impl Animal for Pet {
    fn make_sound(&self) {
        match self {
            Pet::Dog(dog) => dog.make_sound(),
            Pet::Cat(cat) => cat.make_sound(),
        }
    }
}
```

## 主要代码

`#[enum_dispatch]` 宏表明这个 trait 后续将通过 `enum_dispatch` 在不同枚举变体中实现。具体的自动实现行为需要在枚举定义时通过 `#[enum_dispatch(CmdExecutor)]` 明确指定。

```rust
#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}
```

在 Base64SubCommand 枚举类型上使用了 `#[enum_dispatch(CmdExecutor)]` 宏，这意味着 Base64SubCommand 枚举的每个变体都会自动实现 CmdExecutor trait。

```rust
#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(Base64DecodeOpts),
}
```

使用 enum_dispatch 之后，我们原先需要在 match 表达式中根据 Base64SubCommand 的变体调用相应方法的代码现在就可以省略了。

```rust
impl CmdExector for Base64SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            Base64SubCommand::Encode(opts) => opts.execute().await,
            Base64SubCommand::Decode(opts) => opts.execute().await,
        }
    }
}
```

## 单元测试

```bash
cargo nextest run

Compiling rcli v0.1.0 (/Users/I576375/Code/rust/rust-learning/geek-rust-bootcamp/01-rcli)
Finished test [unoptimized + debuginfo] target(s) in 3.56s
Starting 7 tests across 2 binaries (run ID: 631807e4-3d39-4259-bc0b-f6f2f7b4900f, nextest profile: default)
    PASS [   0.010s] rcli cli::tests::test_verify_file
    PASS [   0.009s] rcli process::b64::tests::test_process_encode
    PASS [   0.010s] rcli process::http_serve::tests::test_file_handler
    PASS [   0.010s] rcli process::http_serve::tests::test_file_not_found
    PASS [   0.011s] rcli process::text::tests::test_blake3_sign_verify
    PASS [   0.016s] rcli process::b64::tests::test_process_decode
    PASS [   0.014s] rcli process::text::tests::test_ed25519_sign_verify
------------
 Summary [   0.018s] 7 tests run: 7 passed, 0 skipped
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
