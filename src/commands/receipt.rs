use log::{error, info};

use crate::{
    constants::RECEIPT_TOKEN_FILE,
    files::{check_file_exists, read_file},
    rest_api::ContributeReceipt,
};

pub fn cmd() {
    if !check_file_exists(RECEIPT_TOKEN_FILE) {
        error!("receipt file does not exist. Have you contributed yet?")
    };
    let receipt_as_bytes = read_file(RECEIPT_TOKEN_FILE);

    let receipt: ContributeReceipt = serde_json::from_slice(&receipt_as_bytes).unwrap();

    info!("Receipt : {:?}", receipt);
}
