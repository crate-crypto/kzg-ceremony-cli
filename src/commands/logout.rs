use crate::{
    constants::{ID_TOKEN_FILE, RECEIPT_TOKEN_FILE, SESSION_ID_FILE, USER_ENTROPY_FILE},
    files::remove_file,
};

pub fn cmd() {
    remove_file(ID_TOKEN_FILE);
    remove_file(SESSION_ID_FILE);
    remove_file(USER_ENTROPY_FILE);
    remove_file(RECEIPT_TOKEN_FILE);
}
