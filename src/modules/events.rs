use std::sync::Mutex;
use std::os::raw::c_char;

// Define the signature of the callback function from C#
type EventCallback = extern "C" fn(*const c_char);

lazy_static::lazy_static! {
    static ref CALLBACK_STORE: Mutex<Option<EventCallback>> = Mutex::new(None);
}

pub fn register_callback(callback: EventCallback) {
    let mut lock = CALLBACK_STORE.lock().unwrap();
    *lock = Some(callback);
}

pub fn trigger_event(message: &str) {
    let lock = CALLBACK_STORE.lock().unwrap();
    if let Some(callback) = *lock {
        let c_msg = std::ffi::CString::new(message).unwrap();
        callback(c_msg.as_ptr());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    static CALLED: AtomicBool = AtomicBool::new(false);

    extern "C" fn test_callback(_ptr: *const c_char) {
        CALLED.store(true, Ordering::SeqCst);
    }

    #[test]
    fn test_events() {
        register_callback(test_callback);
        trigger_event("test");
        assert!(CALLED.load(Ordering::SeqCst));
    }
}
