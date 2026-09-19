use axum::{
    Router,
    routing::get
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let router = Router::new().route("/", get(|| async { "Hell o" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, router).await?;

    Ok(())
}
