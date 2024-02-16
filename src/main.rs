mod templates;

use axum::{
    Router,
    routing::get,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = Router::new().route("/", get(crate::templates::index));    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();

    Ok(())
        
}

