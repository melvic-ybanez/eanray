pub fn info(message: &str) {
    log(message, "INFO")
}

pub fn warning(message: &str) {
    log(message, "WARN")
}

pub fn error(message: &str) {
    log(message, "ERROR")
}

pub fn log(message: &str, label: &str) {
    println!("[{label}] {message}")
}