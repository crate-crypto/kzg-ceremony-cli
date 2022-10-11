use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
// These are the command we support in the program
pub enum CliArgs {
    /// Launches a web browser and logs in the user
    Login,
    /// Deletes all of the users credentials on disk
    Logout,
    /// Displays info of the user currently logged in
    User,
    /// Polls the coordinator to see if it is clients
    /// turn to contribute
    Poll,
    /// Checks the status of the ceremony
    Status,
    /// Allows users to view the current transcript
    ViewTranscript,
    // We need the decoding key from the server
    // TODO: Server should send this when they give the IdToken
    // GetJWTKeys,
    /// Captures entropy by reading keyboard strokes
    CaptureEntropy,
    /// Contributes to the ceremony
    Contribute,
    /// Reads the users receipt
    Receipt,
}
