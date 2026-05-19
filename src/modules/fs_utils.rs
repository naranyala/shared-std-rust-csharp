use std::fs;
use std::path::Path;

pub fn ensure_dir(path: &str) -> bool {
    if Path::new(path).exists() {
        return true;
    }
    fs::create_dir_all(path).is_ok()
}

pub fn get_file_extension(path: &str) -> String {
    Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string()
}

#[allow(dead_code)]
pub fn is_file_read_only(path: &str) -> bool {
    fs::metadata(path).map(|m| m.permissions().readonly()).unwrap_or(false)
}
