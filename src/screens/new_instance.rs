use std::path::PathBuf;
use colored::Colorize;
use inquire::{ Confirm, Text };
use nanoid::nanoid;

pub fn mount() {
    let alphabelt: Vec<char> =
        "1234567890qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM"
            .chars()
            .collect();

    let instance_folder = Text::new("Folder name to store the instance:")
        .with_default(&nanoid!(10, alphabelt.as_slice()))
        .prompt()
        .unwrap();
    let instance_name = Text::new("Instance name:")
        .with_default("New MCCL instance")
        .prompt()
        .unwrap();

    let mut executable_location = PathBuf::new();
    loop {
        println!("{} Select your JAR executable server file...", ">".green());

        let location = rfd::FileDialog
            ::new()
            .set_title("Select JAR executable file")
            .add_filter("JAR Executable", &["jar"])
            .set_directory("/")
            .pick_file();

        match location {
            Some(location) => {
                executable_location = location;
                break;
            }
            None => {
                let ask_again = Confirm::new(
                    "You did not choose any file, would you like to stop?"
                )
                    .with_default(false)
                    .prompt()
                    .unwrap();
                if ask_again {
                    break;
                }
            }
        }
    }
}
