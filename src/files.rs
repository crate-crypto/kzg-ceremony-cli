use std::path::PathBuf;

use directories::ProjectDirs;
use log::error;
// We run this check once, at the start incase users are using an esoteric system
pub fn project_dir_check() -> bool {
    let directory = match find_project_dir() {
        Some(dir) => dir,
        None => return false,
    };

    match std::fs::create_dir_all(&directory) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn project_dir() -> PathBuf {
    find_project_dir().unwrap()
}

fn find_project_dir() -> Option<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "EF", "KZG-Ceremony-CLI") {
        return Some(proj_dirs.config_dir().to_path_buf());
    }
    return None;
}

pub fn write_file<F: AsRef<[u8]>>(file_name: &str, file: F) {
    let mut file_path = project_dir();
    file_path.push(file_name);
    std::fs::write(file_path, file).expect("Unable to write file");
}
pub fn read_file(file_name: &str) -> Vec<u8> {
    maybe_read_file(file_name).expect("Unable to read file")
}
pub fn maybe_read_file(file_name: &str) -> Option<Vec<u8>> {
    let mut file_path = project_dir();
    file_path.push(file_name);
    std::fs::read(file_path).ok()
}
pub fn check_file_exists(file_name: &str) -> bool {
    let mut file_path = project_dir();
    file_path.push(file_name);
    file_path.exists()
}

pub fn remove_file(file_name: &str) {
    let mut file_path = project_dir();
    file_path.push(file_name);
    match std::fs::remove_file(file_path) {
        Ok(_) => {}
        Err(err) => error!("{}", err),
    }
}

pub fn append_to_file(file_name: &str, contents: String) {
    use std::io::Write;
    let mut file_path = project_dir();
    file_path.push(file_name);

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();
    write!(file, "{}", contents).expect("could not write to file");
}
