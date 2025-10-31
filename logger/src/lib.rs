use colored::Colorize;

pub fn info(message: &str) {
    println!("[{}] {}", "*".yellow(), message);
}

pub fn success(message: &str) {
    println!("[{}] {}", "+".green(), message);
}

pub fn warn(message: &str) {
    println!("[{}] {}", "-".truecolor(255, 165, 0).bold(), message);
}

pub fn error(message: &str) {
    eprintln!("[{}] {}", "!".red(), message);
}

pub fn sub(message: &str) {
    println!("\t[{}] {}", ">".cyan(), message);
}
