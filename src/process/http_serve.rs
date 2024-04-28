use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", path, addr);

    let state = HttpServeState { path: path.clone() };
    let router = Router::new()
        // nest_service 用于将一个服务嵌套在一个特定的路由前缀下。这意味着所有匹配这个 /tower 前缀的请求都会被转发到指定的服务。
        .nest_service("/tower", ServeDir::new(path))
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

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
            }
            Err(e) => {
                warn!("Failed to read file: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let (status, content) = file_handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.trim().starts_with("[package]"))
    }

    #[tokio::test]
    async fn test_file_not_found() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let (status, _) = file_handler(State(state), Path("not-exist".to_string())).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }
}
