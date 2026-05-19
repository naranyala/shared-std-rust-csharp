use std::ffi::{CStr, CString};
use std::os::raw::c_char;

mod modules {
    pub mod math;
    pub mod text;
    pub mod state;
    pub mod config;
    pub mod fs_utils;
    pub mod sys_info;
    pub mod logging;
    pub mod crypto;
    pub mod shell;
    pub mod net;
    pub mod parallel;
    pub mod regex_engine;
    pub mod events;
}

// --- Common Utilities ---

fn to_cstring(s: String) -> *mut c_char {
    CString::new(s).unwrap().into_raw()
}

fn from_cstring(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    unsafe {
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

// --- Math Module FFI ---

#[unsafe(no_mangle)]
pub extern "C" fn math_add(a: i32, b: i32) -> i32 {
    modules::math::add(a, b)
}

#[unsafe(no_mangle)]
pub extern "C" fn math_multiply(a: i32, b: i32) -> i32 {
    modules::math::multiply(a, b)
}

// --- Text Module FFI ---

#[unsafe(no_mangle)]
pub extern "C" fn text_to_uppercase(name: *const c_char) -> *mut c_char {
    let input = from_cstring(name);
    let result = modules::text::to_uppercase(&input);
    to_cstring(result)
}

#[unsafe(no_mangle)]
pub extern "C" fn text_reverse(name: *const c_char) -> *mut c_char {
    let input = from_cstring(name);
    let result = modules::text::reverse_string(&input);
    to_cstring(result)
}

// --- State Module FFI (Opaque Pointers) ---

#[unsafe(no_mangle)]
pub extern "C" fn session_create(id: u32, name: *const c_char) -> *mut modules::state::UserSession {
    let username = from_cstring(name);
    let session = Box::new(modules::state::UserSession::new(id, username));
    Box::into_raw(session)
}

#[unsafe(no_mangle)]
pub extern "C" fn session_add_score(session_ptr: *mut modules::state::UserSession, points: i32) {
    if !session_ptr.is_null() {
        unsafe {
            (*session_ptr).add_score(points);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn session_get_score(session_ptr: *mut modules::state::UserSession) -> i32 {
    if session_ptr.is_null() {
        return -1;
    }
    unsafe { (*session_ptr).score }
}

#[unsafe(no_mangle)]
pub extern "C" fn session_destroy(session_ptr: *mut modules::state::UserSession) {
    if !session_ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(session_ptr);
        }
    }
}

// --- Global Memory Management ---

#[unsafe(no_mangle)]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

// --- Parallel Processing FFI ---

#[unsafe(no_mangle)]
pub extern "C" fn parallel_sum(data: *const f64, len: usize) -> f64 {
    let slice = unsafe { std::slice::from_raw_parts(data, len) };
    modules::parallel::parallel_sum(slice)
}

#[unsafe(no_mangle)]
pub extern "C" fn parallel_square(data: *mut f64, len: usize) {
    let slice = unsafe { std::slice::from_raw_parts_mut(data, len) };
    modules::parallel::parallel_square(slice)
}

// --- Regex FFI ---

#[unsafe(no_mangle)]
pub extern "C" fn regex_is_match(pattern: *const c_char, text: *const c_char) -> bool {
    modules::regex_engine::is_match(&from_cstring(pattern), &from_cstring(text))
}

#[unsafe(no_mangle)]
pub extern "C" fn regex_replace(pattern: *const c_char, text: *const c_char, repl: *const c_char) -> *mut c_char {
    to_cstring(modules::regex_engine::replace_all(&from_cstring(pattern), &from_cstring(text), &from_cstring(repl)))
}

// --- Event Bridge FFI (The "Vice Versa") ---

#[unsafe(no_mangle)]
pub extern "C" fn register_event_callback(callback: extern "C" fn(*const c_char)) {
    modules::events::register_callback(callback);
}

#[unsafe(no_mangle)]
pub extern "C" fn trigger_rust_event(message: *const c_char) {
    modules::events::trigger_event(&from_cstring(message));
}

// --- Crypto FFI ---

#[unsafe(no_mangle)]
pub extern "C" fn crypto_sha256(input: *const c_char) -> *mut c_char {
    to_cstring(modules::crypto::hash_sha256(&from_cstring(input)))
}

#[unsafe(no_mangle)]
pub extern "C" fn crypto_encode_base64(input: *const c_char) -> *mut c_char {
    to_cstring(modules::crypto::encode_base64(&from_cstring(input)))
}

#[unsafe(no_mangle)]
pub extern "C" fn crypto_decode_base64(input: *const c_char) -> *mut c_char {
    match modules::crypto::decode_base64(&from_cstring(input)) {
        Some(s) => to_cstring(s),
        None => std::ptr::null_mut(),
    }
}

// --- Shell FFI ---

#[unsafe(no_mangle)]
pub extern "C" fn shell_open_url(url: *const c_char) -> bool {
    modules::shell::open_url(&from_cstring(url))
}

// --- Net FFI ---

#[unsafe(no_mangle)]
pub extern "C" fn net_http_get(url: *const c_char) -> *mut c_char {
    match modules::net::http_get(&from_cstring(url)) {
        Ok(s) => to_cstring(s),
        Err(e) => to_cstring(format!("Error: {}", e)),
    }
}

// --- Desktop StdLib FFI ---

// Logging
#[unsafe(no_mangle)]
pub extern "C" fn desktop_log(level: i32, message: *const c_char) {
    let msg = from_cstring(message);
    let lvl = match level {
        0 => modules::logging::LogLevel::Info,
        1 => modules::logging::LogLevel::Warn,
        _ => modules::logging::LogLevel::Error,
    };
    modules::logging::log(lvl, &msg);
}

// System Info
#[unsafe(no_mangle)]
pub extern "C" fn desktop_get_os() -> *mut c_char {
    to_cstring(modules::sys_info::get_os_name())
}

#[unsafe(no_mangle)]
pub extern "C" fn desktop_get_arch() -> *mut c_char {
    to_cstring(modules::sys_info::get_arch())
}

// FS Utils
#[unsafe(no_mangle)]
pub extern "C" fn desktop_ensure_dir(path: *const c_char) -> bool {
    modules::fs_utils::ensure_dir(&from_cstring(path))
}

#[unsafe(no_mangle)]
pub extern "C" fn desktop_get_ext(path: *const c_char) -> *mut c_char {
    to_cstring(modules::fs_utils::get_file_extension(&from_cstring(path)))
}

// Config (Stateful)
#[unsafe(no_mangle)]
pub extern "C" fn config_load(path: *const c_char) -> *mut modules::config::AppConfig {
    let p = from_cstring(path);
    let cfg = Box::new(modules::config::AppConfig::load(&p));
    Box::into_raw(cfg)
}

#[unsafe(no_mangle)]
pub extern "C" fn config_set(cfg_ptr: *mut modules::config::AppConfig, key: *const c_char, val: *const c_char) {
    if !cfg_ptr.is_null() {
        unsafe {
            (*cfg_ptr).set(from_cstring(key), from_cstring(val));
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn config_get(cfg_ptr: *mut modules::config::AppConfig, key: *const c_char) -> *mut c_char {
    if cfg_ptr.is_null() { return std::ptr::null_mut(); }
    let k = from_cstring(key);
    unsafe {
        match (*cfg_ptr).get(&k) {
            Some(v) => to_cstring(v),
            None => std::ptr::null_mut(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn config_save(cfg_ptr: *mut modules::config::AppConfig, path: *const c_char) -> bool {
    if cfg_ptr.is_null() { return false; }
    let p = from_cstring(path);
    unsafe { (*cfg_ptr).save(&p).is_ok() }
}

#[unsafe(no_mangle)]
pub extern "C" fn config_destroy(cfg_ptr: *mut modules::config::AppConfig) {
    if !cfg_ptr.is_null() {
        unsafe { let _ = Box::from_raw(cfg_ptr); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_ffi_math() {
        assert_eq!(math_add(10, 20), 30);
        assert_eq!(math_multiply(10, 20), 200);
    }

    #[test]
    fn test_ffi_text() {
        let name = CString::new("hello").unwrap();
        let upper_ptr = text_to_uppercase(name.as_ptr());
        let upper_str = from_cstring(upper_ptr);
        assert_eq!(upper_str, "HELLO");
        free_string(upper_ptr);
    }

    #[test]
    fn test_ffi_session() {
        let name = CString::new("testuser").unwrap();
        let session = session_create(1, name.as_ptr());
        assert_eq!(session_get_score(session), 0);
        
        session_add_score(session, 50);
        assert_eq!(session_get_score(session), 50);
        
        session_destroy(session);
    }

    #[test]
    fn test_ffi_parallel() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let sum = parallel_sum(data.as_ptr(), data.len());
        assert_eq!(sum, 10.0);
    }

    #[test]
    fn test_ffi_regex() {
        let pat = CString::new(r"^\d+$").unwrap();
        let txt = CString::new("123").unwrap();
        assert!(regex_is_match(pat.as_ptr(), txt.as_ptr()));
    }

    #[test]
    fn test_ffi_events() {
        extern "C" fn dummy_cb(_: *const c_char) {}
        register_event_callback(dummy_cb);
        trigger_rust_event(CString::new("hi").unwrap().as_ptr());
    }
}
