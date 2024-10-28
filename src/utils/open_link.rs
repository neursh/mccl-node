use std::process::Command;

pub fn open(link: &str) {
    let _ = Command::new("cmd").args(["/c", "start", link]).spawn();
}
