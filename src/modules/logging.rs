use std::sync::Mutex;
use std::time::SystemTime;

pub enum LogLevel {
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

lazy_static::lazy_static! {
    static ref LOG_MUTEX: Mutex<()> = Mutex::new(());
}

pub fn log(level: LogLevel, message: &str) {
    let _lock = LOG_MUTEX.lock().unwrap();
    let now = SystemTime::now();
    println!("[{:?}] [{}] {}", now, level.as_str(), message);
}
