use log::info;

use crate::{constants::USER_ENTROPY_FILE, files::append_to_file};

pub fn cmd() {
    info!("Enter random letters on your keyboard and click enter when done.");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    append_to_file(USER_ENTROPY_FILE, input);
    info!("Entropy captured")
}
