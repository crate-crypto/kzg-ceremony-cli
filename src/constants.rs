/// Name of the file that we create to store the id token
pub const ID_TOKEN_FILE: &str = "token.id";
/// Name of the file that we create to store the session id
pub const SESSION_ID_FILE: &str = "session.id";
/// Name of the file that we create to store the contribution receipt
pub const RECEIPT_TOKEN_FILE: &str = "receipt.id";

/// Name of the file that we create to store the user generated entropy
pub const USER_ENTROPY_FILE: &str = "user_entropy.bytes";

/// In seconds, this is the delay between pings
/// that the client initialises to get the chance to
/// contribute
pub const PING_DELAY_TIME: u64 = 10;
