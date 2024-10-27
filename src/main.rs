mod http_block_server;
mod server_launcher;
mod config_reader;
mod utils;

use colored::Colorize;
use inquire::Select;
use utils::pause;
use std::io::Write;

#[rocket::main]
async fn main() -> std::io::Result<()> {
    let config = if let Ok(config_json) = config_reader::json() {
        config_json
    } else {
        print!(
            "{} Can't read configuration in `config.json`. Maybe the file doesn't exist?\n\n{}",
            ">".red().bold(),
            "Press any key to exit...".red()
        );
        std::io::stdout().flush().unwrap();

        pause::pause();
        return Ok(());
    };

    println!(
        "{} Checking for active node running the server...",
        ">".green().bold()
    );
    let server = Select::new(
        "Welcome to MCCL Node! What would you like to do?",
        vec![
            "Start a server instance",
            "Connect to a current running server instance"
        ]
    );
    let selected = server.prompt().unwrap();

    tokio::spawn(async move {
        match server_launcher::start().await {
            Ok(_) => {}
            Err(_) => println!(""),
        }
    });

    http_block_server::run("_name_", 25566).await;

    Ok(())
}
