use std::{ fs, io::{ stdout, Write }, path::{ Path, PathBuf }, time::Duration };
use colored::Colorize;
use indicatif::ProgressBar;
use inquire::{ Confirm, Select, Text };
use nanoid::nanoid;
use crate::{ structs::instance::InstanceConfig, utils::pause };

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

    println!();

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

    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(100));

    let create_folder = format!("instances/{}", instance_folder);
    let executable_filename = executable_location
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    fs::create_dir(&create_folder).unwrap();
    spinner.println(
        format!(
            "{} Created {} folder in instances.",
            ">".green(),
            instance_folder.bright_cyan()
        )
    );

    fs::copy(
        &executable_location,
        format!("{}/{}", create_folder, executable_filename)
    ).unwrap();
    spinner.println(
        format!("{} Copied {}", ">".green(), print_location.bright_cyan())
    );

    let config = InstanceConfig {
        name: instance_name.clone(),
        username: "admin".to_owned(),
        token: nanoid!(48),
        service: None,
        discord_webhook: None,
        local_last_run: 0,
        executable: executable_filename.to_owned(),
        cmd: vec![
            "java".to_owned(),
            "-jar".to_owned(),
            executable_filename.to_owned(),
            "-nogui".to_owned()
        ],
        excluded_lock_structure: vec![],
    };

    let _ = fs::write(
        format!("{}/{}", create_folder, "config.mccl.json"),
        serde_json::to_string_pretty(&config).unwrap()
    );
    let _ = fs::write(format!("{}/{}", create_folder, "eula.txt"), "eula=true");

    spinner.finish();
    print!(
        "{} Created {} successfully!\n\n{}",
        ">".green(),
        instance_name.bright_cyan(),
        "Press any key to go back to main menu...".green()
    );
    let _ = stdout().flush();
    pause::invoke();
}
