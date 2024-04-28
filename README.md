# 第一周：魔法神箭：从 Hello world 到实用的 CLI 工具

* [安装依赖](#安装依赖)
* [Axum, Tokio 和 Tower](#axum-tokio-和-tower)
* [主要代码](#主要代码)
* [验证效果](#验证效果)

## 安装依赖

这些依赖库在Rust项目中有以下作用：

- `tokio`：这是一个 Rust 的异步运行时，提供了异步文件和网络 I/O，定时器，和任务调度等功能。在这个项目中，它被用来处理异步操作，如网络请求和文件操作。特性 `rt`，`rt-multi-thread`，`macros`，`net`，`fs` 分别开启了运行时，多线程运行时，宏，网络和文件系统功能。
- `tracing-subscriber`：这是一个用于 Rust 的应用级追踪和日志记录的库。在这个项目中，它被用来记录和追踪程序的运行情况。特性 `env-filter` 开启了环境变量过滤功能，可以通过设置环境变量来控制日志的输出级别。
- `axum`：这是一个用于构建异步 Web 应用的 Rust 框架。在这个项目中，它被用来处理 HTTP 请求和响应。特性 `http2`，`query`，`tracing` 分别开启了 HTTP/2 支持，查询字符串解析和追踪功能。

```bash
cargo add tokio --features rt --features rt-multi-thread --features macros --features net --features fs
cargo add tracing-subscriber --features env-filter
cargo add axum --features http2 --features query --features tracing
```

## Axum, Tokio 和 Tower

`Tokio`、`Axum` 和 `Tower` 是 Rust 生态系统中用于异步编程和网络服务开发的重要库，它们之间存在密切的关联关系，特别是在构建高性能异步网络应用方面：

- **Tokio**:
    - `Tokio` 是一个 Rust 的异步运行时，专门设计用于处理输入/输出密集型任务，如网络通信、文件操作等。它提供了事件循环、任务调度、非阻塞 IO 等核心功能。
    - `Tokio` 是构建异步网络服务的基础，为包括 `Tower` 和 `Axum` 在内的许多库提供底层的异步能力。

- **Tower**:
    - `Tower` 是一个轻量级的、模块化的服务抽象库，用于构建可复用的网络层组件。它本身并不直接依赖于 `Tokio`，但是为了实现异步处理，`Tower` 的许多实现（特别是和网络服务相关的）会使用 `Tokio` 的异步功能。
    - `Tower` 提供的是一套中间件和服务的抽象，这些可以被集成到任何异步网络应用中，提供如请求路由、负载均衡、错误处理等功能。

- **Axum**:
    - `Axum` 是一个基于 `Tokio` 和 `Tower` 构建的 Web 应用框架。它利用 `Tokio` 的异步运行时来处理并发任务，而 `Tower` 的服务和中间件模型则为 `Axum` 提供了网络层的灵活组合和扩展能力。
    - `Axum` 的设计专注于类型安全和高性能，它通过 `Tokio` 来管理异步任务和事件循环，同时使用 `Tower` 的抽象来组织和优化网络服务处理逻辑。
    - 在 `Axum` 中，每个路由处理函数都被视为一个 `tower::Service`，这使得你可以在 `Axum` 的路由处理函数中使用任何实现了 `tower::Service` trait 的中间件。这种设计使得 `Axum` 可以利用 `Tower` 的生态系统，包括各种现成的中间件，如超时、重试、负载均衡等。

## 主要代码

在 Rust 中，`Arc`（Atomic Reference Counting）是一种线程安全的引用计数指针。它用于在多个线程之间共享所有权的情况。

在代码中，`HttpServeState` 实例被包装在 `Arc` 中，然后被传递给 `Router`。这是因为 `Router` 可能会在多个线程中运行，每个线程都需要访问 `HttpServeState`。`Arc` 确保了在多个线程中安全地共享 `HttpServeState` 实例，而不需要复制数据或担心数据竞争。

当 `Arc` 被克隆时，引用计数会增加。当 `Arc` 的克隆被丢弃时，引用计数会减少。当引用计数达到零时，数据会被清理。

```rust
pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
   let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", path, addr);

    let state = HttpServeState { path };
    let router = Router::new()
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
```

file_handler 函数是处理路由的 Handler，它接受两个参数：`State<Arc<HttpServeState>>` 和 `Path<String>`。

`State` 是一个 Axum 提供的宏，用于从请求中提取状态。`Path` 是一个 Axum 提供的宏，用于从请求中提取路径参数。

```rust
// State(state) 这种写法在 Rust 中称为 pattern matching，是一种解构的写法，可以将 State 中的值解构出来，这里的 state 就是 HttpServeState 的实例。
async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    // path 是基本路径（通过 --dir 参数指定，默认是当前目录），state.path 是用户指定的路径，所以这里使用 join 方法将两个路径拼接在一起。
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    if !p.exists() {
        warn!("File not found: {:?}", p);
        (
            StatusCode::NOT_FOUND,
            format!("File {} note found", p.display()),
        )
    } else {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content)
            },
            Err(e) => {
                warn!("Failed to read file: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}
```

## 验证效果

执行以下命令，启动 HTTP 服务：

```bash
RUST_LOG=info cargo run -- http serve
```

可以执行 test.rest 中的测试用例来验证 HTTP 服务请求。

```bash
### Test static file

GET http://localhost:8080/fixtures/b64.txt

### Test file not found

GET http://localhost:8080/404.file

### Test file format not supported

GET http://localhost:8080/fixtures/ed25519.pk
```
