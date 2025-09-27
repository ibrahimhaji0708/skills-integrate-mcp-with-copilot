// Minimal MCP-like server in Rust using Actix-web
// This example provides a simple HTTP server with a single endpoint
// Requires: actix-web = "4" in Cargo.toml

use actix_web::{get, App, HttpServer, Responder, HttpResponse};

#[get("/mcp")]
async fn mcp() -> impl Responder {
    HttpResponse::Ok().body("MCP server is running!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(mcp))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
