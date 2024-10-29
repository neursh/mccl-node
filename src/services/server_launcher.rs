use std::{ io::{ ErrorKind, Error }, process::Stdio };
use colored::Colorize;
use tokio::{ io::{ AsyncBufReadExt, BufReader }, process::Command };

use crate::structs::instance::Instance;

pub async fn start(instance: &Instance) -> Result<(), Error> {
    let mut java_process = Command::new("java");
    java_process
        .current_dir(instance.path.clone())
        .kill_on_drop(true)
        .stdout(Stdio::piped())
        .args(instance.config.cmd.clone());

    let mut console = java_process.spawn()?;
    let stdout = console.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout).lines();

    let handle = tokio::spawn(async move {
        let status = if let Ok(status) = console.wait().await {
            status
        } else {
            return Err(
                Error::new(
                    ErrorKind::Other,
                    "Failed to stop the server normally"
                )
            );
        };

        if status.success() {
            return Ok(());
        } else {
            return Err(
                Error::new(
                    ErrorKind::Other,
                    "Failed to stop the server normally"
                )
            );
        }
    });

    while let Some(line) = reader.next_line().await? {
        println!("{}", line);
    }

    let runtime_result = handle.await.unwrap();

    runtime_result
}
