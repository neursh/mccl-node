use std::process::Command;

pub fn invoke() {
    let _ = Command::new("cmd").args(["/c", "pause", ">nul"]).status();
}
