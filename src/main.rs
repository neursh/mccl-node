use colored::Colorize;

mod utils;
mod structs;
mod functions;
mod pages;

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

        pages::home::index::mount().await;
    }
}
