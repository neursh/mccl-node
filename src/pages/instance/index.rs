use colored::Colorize;
use inquire::{ Confirm, Select };
use crate::{
    functions::server_launcher,
    pages,
    structs::instance::Instance,
    utils::{ clear, pause },
};

pub async fn mount(instance: &Instance) {
    loop {
        let selected_option = Select::new(
            &format!(
                "Instance {} | {} >",
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

        if selected_option.starts_with("1") {
            host::mount(instance).await;
        }

        if selected_option.starts_with("2") {
            pages::connect::index::mount(instance).await;
        }

        // Launch a local server.
        if selected_option.starts_with("3") {
            let confirm = Confirm::new(
                &format!(
                    "{}{}\n           {}\n           {}\n           {}\nAre you sure?",
                    "Warning".red().bold(),
                    ": Hosting locally WILL NOT sync with the remote service, which means:".bold(),
                    "- Your changes in this session will de-sync from remote service, any no one can join this session, except you.".yellow(),
                    "- When you host publicly again, the program will re-sync with the remote service and REMOVE your changes made locally.".red(),
                    "- This session won't trigger a Discord notification.".green()
                )
            )
                .with_default(false)
                .prompt()
                .unwrap();
            if confirm {
                match launch_handle(&instance).await {
                    Ok(_) => {
                        println!(
                            "\n{} Server stopped normally!\n\n{}",
                            ">".green(),
                            "Press any key to go back...".green()
                        );
                    }
                    Err(_) => {
                        println!(
                            "{} Server stopped prematurely!\n{}",
                            ">".red(),
                            "Press any key to go back...".red()
                        );
                    }
                }

                pause::invoke();
            }
        }

        if selected_option.starts_with("*  E") {
            clear::invoke();
            break;
        }
    }
}

async fn launch_handle(instance: &Instance) -> Result<(), ()> {
    println!("{} Launching server...", ">".green());
    server_launcher::start(instance).await
}
