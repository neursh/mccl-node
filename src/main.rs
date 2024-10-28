mod utils;
mod services;
mod instance;

use colored::Colorize;
use inquire::Select;
use instance::instances_reader;
use services::{ http_block_server, server_launcher };
use utils::pause;
use std::io::Write;

#[rocket::main]
async fn main() -> std::io::Result<()> {
    let instances = if let Ok(instances) = instances_reader::read("instances") {
        instances
    } else {
        print!(
            "{} Can't read the `instances` folder. Maybe the folder doesn't exist?\n\n{}",
            ">".red().bold(),
            "Press any key to exit...".red()
        );
        std::io::stdout().flush().unwrap();

        pause::invoke();
        return Ok(());
    };

    let mut instances_display: Vec<String> = vec![];
    for instance in instances {
        let mut display = instance.config["name"]
            .as_str()
            .to_owned()
            .unwrap()
            .to_string();
        display.push_str(&format!(" ({})", instance.path));

        instances_display.push(display);
    }

    let server = Select::new(
        "Welcome to MCCL Node! Please select an instance to proceed ->",
        instances_display
    );
    let selected = server.prompt().unwrap();

    println!(
        "{} Checking for active node running this instance...",
        ">".green().bold()
    );

    tokio::spawn(async move {
        match server_launcher::start().await {
            Ok(_) => {}
            Err(_) => println!(""),
        }
    });

    http_block_server::run("_name_", 25566).await;

    Ok(())
}
