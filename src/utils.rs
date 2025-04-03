pub fn log(message: &str) {
    println!("[LOG]: {}", message);
}

pub fn error(message: &str) {
    eprintln!("[ERROR]: {}", message);
}