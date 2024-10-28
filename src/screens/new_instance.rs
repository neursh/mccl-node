use std::{ io::{ stdout, Write }, path::{ Path, PathBuf } };
use colored::Colorize;
use inquire::{ Confirm, Select, Text };
use crate::{ services::create_instance, utils::pause };

pub fn mount() {
    let instance_folder = Text::new("Folder name to store the instance:")
        .with_validator(|text: &str| {
            if text.is_empty() {
                return Ok(
                    inquire::validator::Validation::Invalid(
                        "Folder name must not be blank!".into()
                    )
                );
            }
            if Path::new(&format!("instances/{}", text)).exists() {
                return Ok(
                    inquire::validator::Validation::Invalid(
                        "That folder already exists!".into()
                    )
                );
            } else {
                return Ok(inquire::validator::Validation::Valid);
            }
        })
        .prompt()
        .unwrap();

    let instance_name = Text::new("Instance name:")
        .with_default(&instance_folder)
        .prompt()
        .unwrap();

    let executable_location: PathBuf;
    let print_location: &str;
    loop {
        print!(
            "{} Select JAR executable server file for the instance -> ",
            "?".green()
        );
        let _ = stdout().flush();

        let location = rfd::FileDialog
            ::new()
            .set_title("Select JAR executable file")
            .add_filter("JAR Executable", &["jar"])
            .set_directory("/")
            .pick_file();

        match location {
            Some(location) => {
                executable_location = location;
                print_location = executable_location
                    .as_path()
                    .to_str()
                    .unwrap();
                println!(
                    "\r{} Select JAR executable server file for the instance -> {}",
                    ">".green(),
                    print_location.bright_cyan()
                );
                break;
            }
            None => {
                print!("\r");
                stdout().flush().unwrap();
                let ask_again = Confirm::new(
                    "You did not choose any file, would you like to stop?"
                )
                    .with_default(false)
                    .prompt()
                    .unwrap();
                if ask_again {
                    return;
                }
            }
        }
    }

    let eula = Select::new(
        "Do you accept Minecraft's EULA? (https://minecraft.net/eula)",
        vec!["I accept", "I do not accept"]
    )
        .prompt()
        .unwrap();

    if eula != "I accept" {
        print!(
            "{} Can't create an instance without complying with Minecraft's EULA!\n\n{}",
            ">".red(),
            "Press any key to go back to main menu...".red()
        );
        let _ = stdout().flush();
        pause::invoke();

        return;
    }

    create_instance::build(
        instance_folder,
        executable_location.clone(),
        print_location,
        instance_name.clone()
    );

    print!(
        "{} Created {} successfully!\n\n{}",
        ">".green(),
        instance_name.bright_cyan(),
        "Press any key to go back to main menu...".green()
    );
    let _ = stdout().flush();
    pause::invoke();
}
