# 第一周：魔法神箭：从 Hello world 到实用的 CLI 工具

* [安装依赖](#安装依赖)
* [Base64 原理](#base64-原理)
* [进一步拆分模块](#进一步拆分模块)
* [实现 Base64 编解码功能](#实现-base64-编解码功能)
* [单元测试](#单元测试)
* [验证效果](#验证效果)
    * [对输入流进行 base64 编码](#对输入流进行-base64-编码)
    * [对输入文件进行 base64 编码](#对输入文件进行-base64-编码)
    * [对输入流进行 base64 解码](#对输入流进行-base64-解码)
    * [对输入文件进行 base64 解码](#对输入文件进行-base64-解码)
* [参考资料](#参考资料)

## 安装依赖

```bash
cargo add base64
```

## Base64 原理

Base64 是将二进制数据转换为文本数据的一种编码方式，它将 3 个字节的二进制数据编码为 4 个字节的文本数据。Base64 编码表中包含 64 个字符，分别是 `A-Z`、`a-z`、`0-9`、`+` 和 `/`。
标准 Base64 里的 64 个可打印字符是 A-Za-z0-9+/，分别依次对应索引值 0-63。索引表如下：

![](https://chengzw258.oss-cn-beijing.aliyuncs.com/Article/20240425112419.png)

编码时，每 3 个字节一组，共 8 bit * 3 = 24 bit，划分成 4 组，即每 6 bit 代表一个编码后的索引值，划分如下图所示：

![](https://chengzw258.oss-cn-beijing.aliyuncs.com/Article/20240425112518.png)

比如我们对 cat 进行编码：

![](https://chengzw258.oss-cn-beijing.aliyuncs.com/Article/20240425112547.png)

如果待编码内容的字节数不是 3 的整数倍，那需要进行一些额外的处理。

如果最后剩下 1 个字节，那么将补 4 个 0 位，编码成 2 个 Base64 字符，然后补两个 =：

![](https://chengzw258.oss-cn-beijing.aliyuncs.com/Article/20240425112618.png)

UrlSafe 的 Base64 编码是将 standard Base64 编码中的 `+` 和 `/` 替换为 `-` 和 `_`，并且去掉末尾的 `=`。

例如：`ab?\n` Standard Base64 编码的结果为 `YWI/Cg==`，而 UrlSafe base64 编码的结果为为 `YWI_Cg`。

![](https://chengzw258.oss-cn-beijing.aliyuncs.com/Article/20240425113701.png)

## 进一步拆分模块

将 CLI 参数读取、校验的逻辑放在 cli 目录中，将实际的处理逻辑放在 process 目录中。修改完成后的目录结构如下所示：

```bash
 tree src
src
├── cli
│   ├── base64.rs
│   ├── csv.rs
│   ├── genpass.rs
│   └── mod.rs
├── lib.rs
├── main.rs
└── process
    ├── b64.rs
    ├── csv_convert.rs
    ├── gen_pass.rs
    └── mod.rs

2 directories, 10 files
```

## 实现 Base64 编解码功能

接下来说明实现 Base64 编解码功能的主要代码。

get_reader 函数用于根据输入参数获取 Reader 对象，如果输入参数为 `-`，则返回标准输入流，否则返回文件流。
这里之所以使用 `Box<dyn Read>` 类型，是因为使用 `Box<dyn Read>` 使得函数能够统一这两种不同来源的返回类型，提供了很高的灵活性。
由于标准输入和文件输入都实现了 Read trait，我们可以使用 trait 对象来统一这两种类型。

```rust
fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}
```

process_encode 和 process_decode 函数分别用于对输入流进行 base64 编码和解码。这两个函数的实现逻辑基本一致，只是在编码和解码的过程中使用了不同的 Base64Format。Base64Format 是一个枚举类型，用于标识 Base64 编码的格式，包括 Standard 和 UrlSafe 两种格式。

```rust
pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    // 将 reader 中的所有数据读取到 buf 中，直到 reader 返回 EOF（文件结束）。
    reader.read_to_end(&mut buf)?;
    let encode = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    println!("{}", encode);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    // read_to_string 方法用于将读取的数据存储为 String。
    reader.read_to_string(&mut buf)?;
    // trim() 方法用于去除字符串两端的空白字符，包括空格、制表符和换行符。
    let buf = buf.trim();

    let decode = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    let decode = String::from_utf8(decode)?;
    println!("{}", decode);
    Ok(())
}
```

## 单元测试

在 rust 中，单元测试是通过在测试函数上添加 #[test] 属性来实现的。我们可以在测试函数中使用 assert_eq! 宏来断言测试结果是否符合预期。

```rust
#[cfg(test)]
mod tests {
    // super 关键字用于访问当前模块的父模块，而 super::* 则表示访问当前模块的父模块的所有内容。
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File does not exist"));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("not-exist"), Err("File does not exist"));
    }
}
```

我们可以使用 cargo nextest 这个增强版的 cargo test 工具来运行测试用例。首先安装 cargo nextest：

```bash
cargo install cargo-nextest --locked
```

运行全部测试用例：

```bash
cargo nextest run
    Finished test [unoptimized + debuginfo] target(s) in 0.13s
    Starting 3 tests across 2 binaries (run ID: 52e4bafd-bca3-4853-ab52-4cca7338f304, nextest profile: default)
        PASS [   0.006s] rcli process::b64::tests::test_process_encode
        PASS [   0.007s] rcli cli::tests::test_verify_input_file
        PASS [   0.007s] rcli process::b64::tests::test_process_decode
------------
     Summary [   0.008s] 3 tests run: 3 passed, 0 skipped
```

指定运行某个测试用例：

```bash
cargo nextest run -- test_process_decode
    Finished test [unoptimized + debuginfo] target(s) in 0.08s
    Starting 1 test across 2 binaries (2 skipped; run ID: 2b8eb71f-d7c2-49b7-b132-214c479a2644, nextest profile: default)
        PASS [   0.006s] rcli process::b64::tests::test_process_decode
------------
     Summary [   0.006s] 1 test run: 1 passed, 2 skipped
```


## 验证效果

### 对输入流进行 base64 编码

首先对输入的字符串进行 standard base64 编码。

```bash
cargo run -- base64 encode
# 输入
ab?
# 然后回车，再按 ctrl + d 结束输入
# 屏幕会打印 standard 的 base64 编码结果
YWI/Cg==
```

然后再对输入的字符串进行 UrlSafe base64 编码，可以看到 UrlSafe 的编码结果将 `/` 替换为了 `_`，并且去掉了末尾的 `=`。

```bash
cargo run -- base64 encode --format urlsafe

# 输入
ab?
# 然后回车，再按 ctrl + d 结束输入
# 屏幕会打印 UrlSafe 的 base64 编码结果
YWI_Cg
```

### 对输入文件进行 base64 编码

```bash
cargo run -- base64 encode -i README.md

# 输出结果
IyDnrKzkuIDlkajvvJrprZTms5XnpZ7nrq3vvJrku44gSGVsbG8gd29ybGQg5Yiw5a6e55So55qEIENMSSDlt6XlhbcKCiMjIOWGheWuuQoKIyMg5a6J6KOF5L6d6LWWCgpgYGBiYXNoCmNhcmdvIGFkZCBiYXNlNjQKYGBgCgojIyBCYXNlNjQg5Y6f55CGCgohW10oaHR0cHM6Ly9jaGVuZ3p3MjU4Lm9zcy1jbi1iZWlqaW5nLmFsaXl1bmNzLmNvbS9BcnRpY2xlLzIwMjQwNDI1MTExMTU0LnBuZykKCiMjIOmqjOivgeaViOaenAoKIyMjIOWvuei+k+WFpea1gei/m+ihjCBiYXNlNjQg57yW56CBCgrpppblhYjlr7novpPlhaXnmoTlrZfnrKbkuLLov5vooYwgc3RhbmRhcmQgYmFzZTY0IOe8lueggeOAgiAKCmBgYGJhc2gKY2FyZ28gcnVuIC0tIGJhc2U2NCBlbmNvZGUKIyDovpPlhaUKYWI/CiMg54S25ZCO5Zue6L2m77yM5YaN5oyJIGN0cmwgKyBkIOe7k+adn+i+k+WFpQojIOWxj+W5leS8muaJk+WNsCBzdGFuZGFyZCDnmoQgYmFzZTY0IOe8lueggee7k+aenApZV0kvQ2c9PQpgYGAKCueEtuWQjuWGjeWvuei+k+WFpeeahOWtl+espuS4sui/m+ihjCB1cmxzYWZlIGJhc2U2NCDnvJbnoIHvvIzlj6/ku6XnnIvliLAgdXJsc2FmZSDnmoTnvJbnoIHnu5PmnpzlsIYgYC9gIOabv+aNouS4uuS6hiBgX2Ag44CCCgpgYGBiYXNoCmNhcmdvIHJ1biAtLSBiYXNlNjQgZW5jb2RlIC0tZm9ybWF0IHVybHNhZmUKCiMg6L6T5YWlCmFiPwojIOeEtuWQjuWbnui9pu+8jOWGjeaMiSBjdHJsICsgZCDnu5PmnZ/ovpPlhaUKIyDlsY/luZXkvJrmiZPljbAgdXJsc2FmZSDnmoQgYmFzZTY0IOe8lueggee7k+aenApZV0lfQ2c9PQpgYGAKCiMjIyDlr7novpPlhaXmlofku7bov5vooYwgYmFzZTY0IOe8lueggQoKYGBgYmFzaApjYXJnbyBydW4gLS0gYmFzZTY0IGVuY29kZSAtLWZpbGUgUkVBRE1FLm1kCmBgYA==
```

### 对输入流进行 base64 解码

对 standard base64 的编码结果进行解码。

```bash
cargo run -- base64 decode
# 输入
YWI/Cg==
# 然后回车，再按 ctrl + d 结束输入
# 屏幕会打印 standard 的 base64 解码结果
ab?
```

对 UrlSafe base64 的编码结果进行解码。

```bash
cargo run -- base64 decode --format urlsafe
# 输入
YWI_Cg
# 然后回车，再按 ctrl + d 结束输入
# 屏幕会打印 standard 的 base64 解码结果
ab?
```

### 对输入文件进行 base64 解码

```bash
cargo run -- base64 decode -i fixtures/b64.txt

# 输出 README.md 的内容
```

## 参考资料

- [一份简明的 Base64 原理解析](https://zhuanlan.zhihu.com/p/111700349)
- [ASCII 表](https://www.runoob.com/w3cnote/ascii.html)
