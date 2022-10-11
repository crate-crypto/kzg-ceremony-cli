use log::{error, info};
// use rand::Rng;
// use small_powers_of_tau::{
//     sdk::{update_transcript, Transcript, TranscriptJSON, NUM_CEREMONIES},
//     update_proof::UpdateProof,
// };

use kzg_ceremony_crypto::{BatchContribution, BLST};
use rand::{thread_rng, Rng};

use crate::{
    constants::{RECEIPT_TOKEN_FILE, SESSION_ID_FILE, USER_ENTROPY_FILE},
    files::{check_file_exists, read_file, remove_file, write_file},
    rest_api::{ContributeReceipt, RestAPI},
};
// You should not need to call this API manually
// It is only for when you disconnect during contributing
//
pub async fn cmd(api: &RestAPI) {
    unimplemented!()
}

pub async fn contribute(
    api: &RestAPI,
    mut contribution_file: BatchContribution,
) -> Option<ContributeReceipt> {
    // Modify contribution file
    info!("Modifying contribution file");

    if !check_file_exists(USER_ENTROPY_FILE) {
        error!("Before you can contribute, you must add your own entropy");
        error!("use capture-entropy command");
        return None;
    };
    if !check_file_exists(SESSION_ID_FILE) {
        error!("Session Id file missing, please login");
        return None;
    };

    let user_entropy = read_file(USER_ENTROPY_FILE);

    let mut rng = rand::thread_rng();
    let entropy: [u8; 32] = rng.gen();

    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();

    hasher.update(user_entropy);
    hasher.update(entropy);

    // read hash digest and consume hasher
    let result: [u8; 32] = hasher.finalize().into();

    contribution_file
        .add_entropy::<BLST>(&result.into())
        .unwrap();

    info!("Entropy added. Sending file and awaiting response");

    Some(api.send_contribution_file(contribution_file).await)
}
