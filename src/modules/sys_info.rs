use std::env;

pub fn get_os_name() -> String {
    env::consts::OS.to_string()
}

pub fn get_arch() -> String {
    env::consts::ARCH.to_string()
}

#[allow(dead_code)]
pub fn get_env_var(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| "NOT_FOUND".to_string())
}
