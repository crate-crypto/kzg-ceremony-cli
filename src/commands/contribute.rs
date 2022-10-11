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
pub async fn cmd(api: &RestAPI) {}
// pub async fn cmd(api: &RestAPI) {
//     // Check to see if the user has added their own entropy
//     //
//     // TODO: write how users will combine their entropy with the
//     // TODO: cryptographically generated one
//     if !check_file_exists(USER_ENTROPY_FILE) {
//         error!("Before you can contribute, you must add your own entropy");
//         error!("use capture-entropy command");
//         return;
//     };
//     if !check_file_exists(SESSION_ID_FILE) {
//         error!("Session Id file missing, please login");
//         return;
//     };

//     // Fetch latest transcript from coordinator
//     info!("Fetching current transcript");
//     let transcript_json = api.get_transcript().await;

//     //
//     info!("Adding contribution");
//     let transcript: Option<Transcript> = (&transcript_json).into();
//     let transcript = match transcript {
//         Some(transcript) => transcript,
//         None => {
//             error!("Could not deserialise transcript");
//             return;
//         }
//     };

//     // Compute four secrets using RNG
//     let secrets = random_hex_strs();
//     let (transcript, proofs) = match update_transcript(transcript, secrets) {
//         Some((transcript, update_proof)) => (transcript, update_proof),
//         None => {
//             error!("Failed to update the transcript");
//             return;
//         }
//     };

//     info!("Sending transcript. Awaiting response");
//     let response = api
//         .send_transcript(ContributePayload {
//             state: TranscriptJSON::from(&transcript),
//             witness: update_proofs_to_json_arrays(proofs),
//         })
//         .await;

//     match response {
//         ContributeResponse::Error(err) => error!("Contribution failed: {}", err),
//         ContributeResponse::Receipt(receipt) => {
//             info!("Contribution was a success!");
//             info!("Receipt: {}", receipt);
//             write_file(RECEIPT_TOKEN_FILE, &receipt);
//             remove_file(SESSION_ID_FILE);
//         }
//     }
// }
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

// fn update_proofs_to_json_arrays(
//     proofs: [UpdateProof; NUM_CEREMONIES],
// ) -> [[String; 2]; NUM_CEREMONIES] {
//     proofs.map(|proof| proof.serialise())
// }

// fn random_hex_strs() -> [String; NUM_CEREMONIES] {
//     let mut rng = rand::thread_rng();

//     let mut secrets: [String; NUM_CEREMONIES] = [
//         String::default(),
//         String::default(),
//         String::default(),
//         String::default(),
//     ];

//     for i in 0..NUM_CEREMONIES {
//         // We use 64 bytes for the secret to reduce bias when reducing
//         let mut bytes = [0u8; 64];
//         rng.fill(&mut bytes);

//         let mut hex_string = hex::encode(&bytes);
//         // prepend 0x because this is standard in ethereum
//         hex_string.insert_str(0, "0x");
//         secrets[i] = hex_string
//     }

//     secrets
// }
