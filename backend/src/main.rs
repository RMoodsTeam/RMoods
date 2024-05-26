use axum::{routing::get, Json, Router};
use log::info;
use serde_json::{json, Value};

/// The port on which the server starts.
const PORT: u16 = 3000;

/// Returns a welcome message and a link to our documentation   
async fn hello() -> Json<Value> {
    json!({
        "message" : "Welcome to the RMoods Backend!",
            "docs": "https://xmoods.github.io/XMoods/backend/xmoods_backend/index.html"
    }).into()
}

/// Entry point of the XMoods server.
/// Initializes the HTTP server and runs it.
#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    
    let app = Router::new().route("/", get(hello));

    let addr = format!("0.0.0.0:{PORT}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    info!("Starting the XMoods server");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_test() {
        assert_eq!(2 + 2, 4);
    }
}
