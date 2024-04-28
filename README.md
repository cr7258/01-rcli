# 第一周：魔法神箭：从 Hello world 到实用的 CLI 工具

* [添加依赖](#添加依赖)
* [Tower, tower-http 和 Axum 的关系](#tower-tower-http-和-axum-的关系)
* [主要代码](#主要代码)
* [单元测试](#单元测试)
* [验证效果](#验证效果)

## 添加依赖

```bash
cargo add tower-http --features compression-full --features cors --features trace --features fs
```

## Tower, tower-http 和 Axum 的关系

`tower-http` 是一个构建在 `Tower` 框架之上的库，专门用于处理 HTTP 相关的功能。它是 `Tower` 生态系统的一部分，提供了多种 HTTP 中间件和工具，使得开发者可以更有效地构建 HTTP 服务。

以下是 `tower-http` 与 `Tower` 和 `Axum` 的关系概述：

- **与 Tower 的关系**：`tower-http` 直接扩展了 `Tower` 的功能，提供了专门针对 HTTP 服务的中间件和工具。这些中间件可以处理各种 HTTP 请求和响应的场景，例如日志记录、压缩、认证等。由于 `tower-http` 是基于 `Tower` 构建的，它自然地融入了 `Tower` 的设计哲学和使用方式。
- **与 Axum 的关系**：由于 `Axum` 也是建立在 `Tower` 之上的 Web 框架，`tower-http` 中的功能可以被 `Axum` 利用来增强其 HTTP 处理能力。例如，使用 `tower-http` 提供的中间件来实现请求日志记录、请求超时处理等。`Axum` 用户可以直接在他们的应用中使用 `tower-http` 的中间件，以提升应用的性能和功能。

## 主要代码

使用 `tower-http` 提供的 `ServeDir` 中间件，可以方便地将指定目录下的文件提供给 HTTP 客户端，比我们自己像上一节那样写 file_handler 来处理更加简单。

```bash
nest_service("/tower", ServeDir::new(path))
```

## 单元测试

```bash
cargo nextest run
   Compiling rcli v0.1.0 (/Users/I576375/Code/rust/rust-learning/geek-rust-bootcamp/01-rcli)
    Finished test [unoptimized + debuginfo] target(s) in 1.99s
    Starting 7 tests across 2 binaries (run ID: d34f6e68-f425-4f42-a7a9-31988fd1e410, nextest profile: default)
        PASS [   0.009s] rcli cli::tests::test_verify_file
        PASS [   0.008s] rcli process::http_serve::tests::test_file_not_found
        PASS [   0.009s] rcli process::b64::tests::test_process_encode
        PASS [   0.009s] rcli process::http_serve::tests::test_file_handler
        PASS [   0.011s] rcli process::b64::tests::test_process_decode
        PASS [   0.010s] rcli process::text::tests::test_blake3_sign_verify
        PASS [   0.010s] rcli process::text::tests::test_ed25519_sign_verify
------------
     Summary [   0.013s] 7 tests run: 7 passed, 0 skipped
```

## 验证效果

执行以下命令启动 HTTP 文件服务器：

```bash
RUST_LOG=info cargo run -- http serve
```

以下两个请求的效果一样，分别测试了只使用 Axum 和使用了 tower-http 的效果：

```bash
### Test static file

GET http://localhost:8080/fixtures/b64.txt

### Test static file with tower-http

GET http://localhost:8080/tower/fixtures/b64.txt
```
