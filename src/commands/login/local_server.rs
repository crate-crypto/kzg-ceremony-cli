use axum::{extract::Query, response::Html, routing::get, Extension, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::sync::{mpsc::Sender, oneshot::Receiver};

pub(crate) async fn launch_server(shutdown_signal: Receiver<()>, info_tx: Sender<UserProfile>) {
    // build our application with a route
    let app = Router::new()
        .route("/auth/callback/eth", get(handler))
        .layer(Extension(info_tx));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let server = axum::Server::bind(&addr).serve(app.into_make_service());

    let graceful = server.with_graceful_shutdown(async {
        shutdown_signal.await.ok();
    });

    // Await the `server` receiving the signal...
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct UserProfile {
    pub session_id: String,
    pub sub: String,
    pub nickname: String,
    pub provider: String,
    pub exp: u64,
}

async fn handler(
    Query(query): Query<UserProfile>,
    Extension(info_tx): Extension<Sender<UserProfile>>,
) -> Html<&'static str> {
    let _ = info_tx.send(query).await;

    Html("<h2>Login successful. Please return back to the terminal!</h2>")
}
