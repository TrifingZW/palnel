use std::net::SocketAddr;

use axum::Router;
use tracing::info;

/// 启动 HTTP 服务器（主应用逻辑）
pub async fn http(app: Router, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Starting HTTP main server on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

/// 启动 HTTP 服务器（重定向至 HTTPS）
#[cfg(not(debug_assertions))]
pub async fn redirect(port: u16) -> anyhow::Result<()> {
    use axum::{extract::Request, response::Redirect};

    let app = Router::new().fallback(|request: Request| async move {
        let host =
            request.headers().get("host").and_then(|h| h.to_str().ok()).unwrap_or("localhost");
        let path_and_query = request.uri().path_and_query().map(|pq| pq.as_str()).unwrap_or("/");
        let redirect_url = format!("https://{}{}", host, path_and_query);
        info!("Redirecting HTTP to HTTPS: {}", redirect_url);
        Redirect::temporary(&redirect_url)
    });
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Starting HTTP redirect server on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

/// 启动 HTTPS 服务器（主应用逻辑）
#[cfg(not(debug_assertions))]
pub async fn https(
    app: Router,
    port: u16,
    tls: common::config::ServerTlsConfig,
) -> anyhow::Result<()> {
    use axum_server::tls_openssl::OpenSSLConfig;

    let tls_config = OpenSSLConfig::from_pem_file(tls.cert, tls.key)?;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Starting HTTPS main server on https://{}", addr);
    axum_server::bind_openssl(addr, tls_config)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;
    Ok(())
}
