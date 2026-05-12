use anyhow::Context;
use axum::{
    Router,
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
};
use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    str::FromStr,
    sync::Arc,
    time::Duration,
};
use tokio::fs;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    root: PathBuf,
}

impl HttpServeState {
    fn new(path: PathBuf) -> anyhow::Result<Arc<Self>> {
        let root = path
            .canonicalize()
            .with_context(|| format!("Cannot resolve root path: {:?}", path))?;
        Ok(Arc::new(Self { root }))
    }

    /// Safely resolve a request path under the root, blocking traversal.
    fn safe_join(&self, rel: &str) -> Option<PathBuf> {
        let rel = rel.trim_start_matches('/');
        // Block null bytes immediately
        if rel.contains('\0') {
            return None;
        }
        let joined = self.root.join(rel);
        // canonicalize only succeeds if the path exists
        let canonical = joined.canonicalize().ok()?;
        canonical.starts_with(&self.root).then_some(canonical)
    }
}

pub async fn process_http_serve(path: PathBuf, ip: &str, port: u16) -> anyhow::Result<()> {
    let ip = IpAddr::from_str(ip).with_context(|| format!("Invalid IP address: {}", ip))?;
    let addr = SocketAddr::from((ip, port));
    let state = HttpServeState::new(path)?;

    info!("Root : {:?}", state.root);
    info!("Addr : http://{addr}");

    let middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
        .layer(CompressionLayer::new())
        .layer(SetResponseHeaderLayer::overriding(
            // basic hardening
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ));

    let router = Router::new()
        .route("/health", get(health_handler))
        .route("/{*path}", get(file_handler))
        .fallback_service(ServeDir::new(&state.root).fallback(ServeFile::new("404.html"))) // with tower-http
        .layer(middleware)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("Failed to bind port {} (may be in use)", port))?;

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, "Healthy OK!")
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(rel_path): Path<String>,
) -> Response {
    let Some(target) = state.safe_join(&rel_path) else {
        warn!("Blocked or missing: {rel_path}");
        return (StatusCode::NOT_FOUND, "404 Not Found").into_response();
    };

    // Stream the file instead of loading it all into memory
    let file = match fs::File::open(&target).await {
        Ok(f) => f,
        Err(e) => {
            warn!("Open error {:?}: {e}", target);
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    };

    let meta = match file.metadata().await {
        Ok(m) => m,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    if meta.is_dir() {
        // Try to serve index.html inside the directory
        let index = target.join("index.html");
        return match fs::read(index).await {
            Ok(bytes) => html_response(bytes),
            Err(_) => directory_listing(&state.root, &target, &rel_path).await,
        };
    }

    let mime = mime_guess::from_path(&target)
        .first_or_octet_stream()
        .to_string();

    let content_length = meta.len();
    let stream = tokio_util::io::ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_str(&mime).unwrap());
    headers.insert(header::CONTENT_LENGTH, HeaderValue::from(content_length));
    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=3600"),
    );

    (StatusCode::OK, headers, body).into_response()
}

async fn directory_listing(_root: &PathBuf, dir: &PathBuf, rel: &str) -> Response {
    let mut entries = match fs::read_dir(dir).await {
        Ok(e) => e,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let mut rows = String::new();
    while let Ok(Some(entry)) = entries.next_entry().await {
        let name = entry.file_name().to_string_lossy().into_owned();
        let is_dir = entry.file_type().await.map(|t| t.is_dir()).unwrap_or(false);
        let suffix = if is_dir { "/" } else { "" };
        let href = format!("/{}/{}{}", rel.trim_matches('/'), name, suffix);
        rows.push_str(&format!("<li><a href=\"{href}\">{name}{suffix}</a></li>\n"));
    }

    let html = format!(
        "<!DOCTYPE html><html><head>\
         <meta charset=\"utf-8\">\
         <title>Index of /{rel}</title>\
         <style>body{{font-family:monospace;padding:2rem}}a{{display:block;padding:.2rem 0}}</style>\
         </head><body>\
         <h2>Index of /{rel}</h2><ul>{rows}</ul>\
         </body></html>"
    );

    html_response(html.into_bytes())
}

fn html_response(bytes: Vec<u8>) -> Response {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/html; charset=utf-8"),
    );
    (StatusCode::OK, headers, bytes).into_response()
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c    => info!("Ctrl+C received, shutting down…"),
        _ = terminate => info!("SIGTERM received, shutting down…"),
    }
}
