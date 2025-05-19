use colored::Colorize;
use functions::instances;
use screens::{ home_menu, instance_menu };
use utils::pause;

mod utils;
mod screens;
mod structs;
mod functions;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    loop {
        println!(
            "{}\n{}     {}     {}\n{}   {}   {}\n{}",
            "┌─────────────────────────────────┐".green(),
            "│".green(),
            "Welcome to MCCL Node :3".bright_cyan().bold(),
            "│".green(),
            "|".green(),
            "github.com/neursh/mccl-node".bright_magenta(),
            "│".green(),
            "└─────────────────────────────────┘".green()
        );

        let instances_fetch = if let Ok(cache) = instances::fetch() {
            cache
        } else {
            println!(
                "The program ran into a problem with instances reading. Press any key to try again..."
            );
            pause::invoke();
            continue;
        };

        let selected_option = home_menu::mount(&instances_fetch);

        if selected_option.starts_with("*") {
            // New instance
        } else {
            // Split off some of the prefix characters from styling the text.
            let index =
                selected_option
                    .split(".")
                    .next()
                    .unwrap()[4..]
                    .parse::<usize>()
                    .unwrap() - 1;

            let selected_instance = instances_fetch.get(index).unwrap();

            instance_menu::mount(selected_instance).await;
        }
    }
}
