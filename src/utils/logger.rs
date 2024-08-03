use chrono::Local;
use colored::Colorize;

pub fn info(msg: &str) {
    let date = Local::now();
    let print_text = format!("[{}]: {}", date.format("%H:%M:%S"), msg.bright_black());
    // append_to_file(&print_text);
    println!("{}", print_text);
}

pub fn warning(msg: &str) {
    let date = Local::now();
    let print_text = format!("[{}]: {}", date.format("%H:%M:%S"), msg.bright_yellow());
    // append_to_file(&print_text);
    println!("{}", print_text);
}

pub fn error(msg: &str) {
    let date = Local::now();
    let print_text = format!("[{}]: {}", date.format("%H:%M:%S"), msg.bright_red());
    // append_to_file(&print_text);
    println!("{}", print_text);
}

#[allow(dead_code)]
fn append_to_file(_msg: &str) {
    todo!();
}