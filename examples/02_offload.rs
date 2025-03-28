use axum::{Router, http::StatusCode, response::IntoResponse, routing::post};
use forbidden_text_check::contains_forbidden_text_static;
use std::net::SocketAddr;
use tokio::spawn;

#[tokio::main]
async fn main() {
    // The main() entrypoint thread is special and doing work directly in there is
    // less efficient in some scenarios. Spawning a new task will move the work off to
    // a worker thread, where it may be executed more efficiently.
    spawn(async move {
        let app = Router::new().route("/check", post(check));

        let addr = SocketAddr::from(([0, 0, 0, 0], 1234));
        println!("Server running on http://{}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    })
    .await
    .unwrap();
}

async fn check(body: String) -> impl IntoResponse {
    if contains_forbidden_text_static(&body) {
        (StatusCode::OK, "true")
    } else {
        (StatusCode::OK, "false")
    }
}
