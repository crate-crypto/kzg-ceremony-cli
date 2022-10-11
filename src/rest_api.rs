use kzg_ceremony_crypto::{BatchContribution, G2};
use serde::{Deserialize, Serialize};

use crate::{constants::SESSION_ID_FILE, files::read_file};

pub struct RestAPI {
    // Base Url with the trailing forward slash
    base_url: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthRequestLink {
    pub eth_auth_url: String,
    pub github_auth_url: String,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(crate) struct AuthBody {
    pub id_token: IdToken,

    pub session_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Receipt {
    pub id_token: IdToken,
    pub witness: Vec<G2>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IdToken {
    pub sub: String,
    pub nickname: String,
    // The provider whom the client used to login with
    // Example: Google, Ethereum, Facebook
    pub provider: String,
    pub exp: u64,
}

impl Default for RestAPI {
    fn default() -> Self {
        RestAPI::new(String::from("https://kzg-ceremony-sequencer-dev.fly.dev/"))
    }
}

impl RestAPI {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }
    // Request authorization link so users can sign in
    // using their social provider
    pub async fn request_auth_link(&self) -> AuthRequestLink {
        const ROUTE: &str = "auth/request_link/";
        let mut url = self.base_url.to_owned();
        url.push_str(ROUTE);
        url.push_str("?redirect_to=http://127.0.0.1:3000/auth/callback/eth");

        let client = reqwest::Client::new();
        let request_link: AuthRequestLink = client
            .get(url)
            .send()
            .await
            .unwrap()
            .json::<AuthRequestLink>()
            .await
            .unwrap();

        request_link
    }

    pub(crate) async fn try_contribute(&self) -> SlotJoinResponse {
        const ROUTE: &str = "lobby/try_contribute/";
        let mut url = self.base_url.to_owned();
        url.push_str(ROUTE);

        let session_id_bytes = read_file(SESSION_ID_FILE);
        let session_id = String::from_utf8(session_id_bytes).unwrap();

        let client = reqwest::Client::new();

        let slot_join_payload = client
            .post(url)
            .bearer_auth(session_id)
            .send()
            .await
            .unwrap()
            .json::<SlotJoinResponse>()
            .await
            .unwrap();

        slot_join_payload
    }

    pub(crate) async fn send_contribution_file(
        &self,
        payload: BatchContribution,
    ) -> ContributeReceipt {
        const ROUTE: &str = "contribute/";
        let mut url = self.base_url.to_owned();
        url.push_str(ROUTE);

        let session_id_bytes = read_file(SESSION_ID_FILE);
        let session_id = String::from_utf8(session_id_bytes).unwrap();
        let client = reqwest::Client::new();

        let contribute_response = client
            .post(url)
            .bearer_auth(session_id)
            .json(&payload)
            .send()
            .await
            .unwrap()
            .json::<ContributeReceipt>()
            .await
            .unwrap();

        contribute_response
    }
    pub(crate) async fn get_ceremony_status(&self) -> StatusResponse {
        const ROUTE: &str = "info/status/";
        let mut url = self.base_url.to_owned();
        url.push_str(ROUTE);

        let client = reqwest::Client::new();
        let status_response = client
            .get(url)
            .send()
            .await
            .unwrap()
            .json::<StatusResponse>()
            .await
            .unwrap();

        status_response
    }

    pub fn link_to_current_transcript(&self) -> String {
        const ROUTE: &str = "info/current_transcript/";
        let mut url = self.base_url.to_owned();
        url.push_str(ROUTE);
        url
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PingResponse {
    #[serde(rename = "error")]
    Error(String),
    #[serde(rename = "deadline")]
    Deadline(u64),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]

pub enum SlotJoinResponse {
    Error(String),
    Success(BatchContribution),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContributeResponse {
    Error(String),
    Receipt(Receipt),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signature(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct ContributeReceipt {
    receipt: String,
    signature: Signature,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatusResponse {
    #[serde(rename = "error")]
    Error {
        error: String,
    },
    Success(SuccessStatusResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessStatusResponse {
    pub lobby_size: usize,
    pub num_contributions: usize,
}
