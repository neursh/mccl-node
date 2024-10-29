use colored::Colorize;
use inquire::{ Confirm, Select };
use crate::{
    services::server_launcher,
    structs::instance::Instance,
    utils::{ clear, pause },
};

pub async fn mount(instance: Instance) {
    loop {
        let selected_option = Select::new(
            &format!(
                "Instance {} | {} ->",
                instance.config.name.bright_cyan(),
                instance.config.username.bright_cyan()
            ),
            vec![
                "1. Host this instance".bold(),
                "2. Connect to an active node".bold(),
                "3. Host this instance locally".italic(),
                "*  Settings".normal(),
                "*  Exit instance".normal()
            ]
        )
            .prompt()
            .unwrap();

        if selected_option.starts_with("3") {
            let confirm = Confirm::new(
                "Hosting locally only allows you to access the server on your computer. Are you sure?"
            )
                .with_default(false)
                .prompt()
                .unwrap();
            if confirm {
                println!("{} Launching server...\n", ">".green());
                match server_launcher::start(&instance).await {
                    Ok(_) => {
                        println!(
                            "\n{} Server stopped normally!\n\n{}",
                            ">".green(),
                            "Press any key to go back...".green()
                        );
                    }
                    Err(_) => {
                        println!(
                            "{} Server stopped prematurely!\n\n{}",
                            ">".red(),
                            "Press any key to go back...".red()
                        );
                    }
                }
                pause::invoke();
            }

            clear::invoke();
        }

        if selected_option.starts_with("*  Exit") {
            break;
        }
    }
}
