use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

mod api;
mod auth;

/// Index route of the XMoods server.
/// # Returns
/// JSON object with a welcome message and a link to the API documentation.
#[get("/")]
async fn index() -> impl Responder {
    let res = json!({
        "message": "Welcome to XMoods API!",
        "documentation": "https://xmoods.github.io/XMoods/doc/xmoods_api/index.html"
    });
    HttpResponse::Ok().json(res)
}

/// Entry point of the XMoods server.
/// Initializes the HTTP server and runs it.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    
    HttpServer::new(|| {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(index)
            .service(web::scope("/api").configure(api::router))
            .service(web::scope("/auth").configure(auth::router))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_test() {
        assert!(2 + 2 == 4)
    }
}
