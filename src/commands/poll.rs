use crate::{
    commands::contribute::contribute,
    constants::{PING_DELAY_TIME, RECEIPT_TOKEN_FILE},
    files::write_file,
    rest_api::{RestAPI, SlotJoinResponse},
};
use log::{error, info};
use std::time::Duration;

pub async fn cmd(api: &RestAPI) {
    let mut interval = tokio::time::interval(Duration::from_secs(PING_DELAY_TIME));

    loop {
        let response = api.try_contribute().await;
        match response {
            SlotJoinResponse::Error(err) => {
                error!("{}", err)
            }
            SlotJoinResponse::Success(message) => {
                info!("Now Contributing!");
                let response = contribute(api, message).await;
                match response {
                    Some(res) => {
                        write_file(RECEIPT_TOKEN_FILE, serde_json::to_string(&res).unwrap())
                    }
                    None => {
                        error!("could not contribute");
                        break;
                    }
                }

                break;
            }
        }
        info!("Pinging again in {} seconds", PING_DELAY_TIME);
        interval.tick().await;
    }
}
