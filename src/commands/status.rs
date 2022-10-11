use crate::rest_api::{RestAPI, StatusResponse};
use log::{error, info};

pub async fn cmd(api: &RestAPI) {
    let status = api.get_ceremony_status().await;
    let status = match status {
        StatusResponse::Error { error: err } => {
            error!("{}", err);
            return;
        }
        StatusResponse::Success(status) => status,
    };
    info!("lobby size : {} ", status.lobby_size);
    info!("number of contributions : {} ", status.num_contributions);
}
