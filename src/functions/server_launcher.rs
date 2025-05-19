use colored::Colorize;
use tokio::process::Command;

use crate::structs::instance::Instance;

pub async fn start(instance: &Instance) -> Result<(), ()> {
    let debug_args = format!(
        "{} {}",
        instance.config.java_runtime,
        instance.config.args.join(" ")
    );

    let mut child = match
        Command::new(instance.config.java_runtime.clone())
            .current_dir(instance.path.clone())
            .kill_on_drop(true)
            .args(instance.config.args.clone())
            .spawn()
    {
        Ok(proc) => proc,
        Err(message) => {
            println!(
                "{}\n{:?}\n\n{}\n\"{}\"",
                "Something when wrong when trying to launch the server:"
                    .red()
                    .bold(),
                message,
                "Arguments used:".yellow().bold(),
                debug_args
            );
            return Err(());
        }
    };

    match child.wait().await {
        Ok(status) => {
            if status.success() {
                return Ok(());
            } else {
                match status.code() {
                    Some(code) =>
                        println!(
                            "{}{:?}\n\n{}\n\"{}\"",
                            "Received a bad status code from the server's process: "
                                .red()
                                .bold(),
                            code,
                            "Arguments used:".yellow().bold(),
                            debug_args
                        ),
                    None =>
                        println!(
                            "{}\n\n{}\n\"{}\"",
                            "Process terminated by signal.".yellow().bold(),
                            "Arguments used:".yellow().bold(),
                            debug_args
                        ),
                }

                return Err(());
            }
        }
        Err(_) => {
            println!(
                "{}\n\n{}\n\"{}\"",
                "Unknown exit reason.".yellow().bold(),
                "Arguments used:".yellow().bold(),
                debug_args
            );
            return Err(());
        }
    }
}
