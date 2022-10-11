use crate::{
    commands::{capture_entropy, poll, user},
    constants::{ID_TOKEN_FILE, SESSION_ID_FILE},
    files::{check_file_exists, write_file},
    rest_api::RestAPI,
};

pub mod local_server;
use local_server::{launch_server, UserProfile};
use log::{info, warn};

pub async fn cmd(api: &RestAPI) {
    // Check if there is already a session id
    if check_file_exists(&SESSION_ID_FILE) && check_file_exists(ID_TOKEN_FILE) {
        warn!("Found session id and token id on disk. Skipping login.");
        return;
    }

    // Before a user logs in, they will need to supply entropy
    // Every time a user logs in, they will need to supply entropy
    // TODO: We could change this so that it only asks if the file is not there
    capture_entropy::cmd();

    let request_link = api.request_auth_link().await;

    // TODO: safari does not work when we want to sign in with ethereum
    // open::that(request_link.eth_auth_url).unwrap();
    open::with(request_link.eth_auth_url, "firefox").unwrap();

    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    let (info_tx, mut info_rx) = tokio::sync::mpsc::channel::<UserProfile>(1);

    // launch server
    tokio::spawn(async move {
        info!("Launching browser...");
        info!("Complete the sign in process in your browser and return");
        launch_server(shutdown_rx, info_tx).await;
    });

    if let Some(message) = info_rx.recv().await {
        // send shutdown signal
        let _ = shutdown_tx.send(());

        let id_token_serialised = serde_json::to_string(&message).unwrap();
        write_file(ID_TOKEN_FILE, id_token_serialised);
        write_file(SESSION_ID_FILE, &message.session_id);
    }

    info!("Login successful");

    user::cmd();

    info!("Polling for a chance to contribute");
    // Now we immediately start polling for a spot
    poll::cmd(api).await;
}
