use std::{ fs, path::PathBuf, time::Duration };
use nanoid::nanoid;
use colored::Colorize;
use indicatif::ProgressBar;

use crate::structs::instance::InstanceConfig;

pub fn build(
    instance_folder: String,
    executable_location: PathBuf,
    print_location: &str,
    instance_name: String
) {
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
    spinner.println(
        format!(
            "{} Created {} and {}",
            ">".green(),
            "config.mccl.json".bright_cyan(),
            "eula.txt".bright_cyan()
        )
    );

    spinner.finish();
}
