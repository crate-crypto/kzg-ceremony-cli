use jsonwebtoken::{decode, DecodingKey, Validation};
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::{
    constants::ID_TOKEN_FILE,
    files::{check_file_exists, read_file},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct IdToken {
    pub(crate) sub: String,
    pub nickname: String,
    pub provider: String,
    pub exp: u64,
}

pub fn cmd() {
    // TODO: This uses HMAC currently, we should get a public key from
    // TODO the coordinator instead
    // let key = DecodingKey::from_secret(b"secret");
    if !check_file_exists(ID_TOKEN_FILE) {
        error!("Not Logged in");
        return;
    }
    let bytes = read_file(ID_TOKEN_FILE);
    let encoded_id_token = String::from_utf8(bytes).expect("could not read id token from file");
    let token_data: IdToken = serde_json::from_str(&encoded_id_token).unwrap();

    info!(
        "Welcome {}. Your ID provider is {}",
        token_data.nickname, token_data.provider,
    )
}
