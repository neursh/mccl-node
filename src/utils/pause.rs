use std::process::Command;

pub fn pause() {
    let _ = Command::new("cmd").args(["/c", "pause", ">nul"]).status();
}
