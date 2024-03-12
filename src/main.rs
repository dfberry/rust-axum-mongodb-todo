
mod database;
mod database_error;
mod list;
mod route;

use hyper::{Request, Response};
use std::sync::Arc;
use tower::{Service, ServiceExt};
use tower_http::cors::CorsLayer;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};

use mongodb::Database;

use dotenv::dotenv;
use route::create_router;

pub struct AppState {
    db: mongodb::Database,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // get PORT from env 
    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());

  
    let db = match database::get_database().await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to get database: {}", e);
            std::process::exit(1);
        }
    };

    let mut app = create_router(Arc::new(AppState { db: db.clone() }));

    if let Ok(cors_required) = std::env::var("SERVER_CORS_REQUIRED") {
        if cors_required.to_lowercase() == "true" {
            let cors = CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
                .allow_credentials(true)
                .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
            app = app.layer(cors);
        }
    }
    println!("🚀 Server started successfully on port {}", port);
    let listener = match tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to address: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}
