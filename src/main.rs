mod pieces;
use pieces::rand_pieces;
use std::net::SocketAddr;
use axum::{
  extract::Json,
  response::IntoResponse,
  routing::get,
  Router
};

// --- Handlers (API Logic) ---
// Handler for GET /start
async fn get_hello_world() -> String {
    "Hello World!".to_string()
}

async fn get_rand_pieces() -> impl IntoResponse {
  let row = rand_pieces();
  Json(row)
}

//  --- Main Server Setup ---
//  The `#[tokio::main]` attribute makes this function the entry point
//  and initializes the Tokio runtime.
#[tokio::main]
async fn main() {
    // Set up tracing for better logs
    tracing_subscriber::fmt::init();

    // Build the application router
    let app = Router::new()
        // Define a GET route for /start path
        .route("/check", get(get_hello_world))
        .route("/get_start_position", get(get_rand_pieces));

    // Define the address to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);

    // Start the server
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}