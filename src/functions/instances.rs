use std::fs;

use colored::Colorize;

use crate::{ structs::instance::Instance };

use super::config_reader;

pub fn fetch() -> Result<Vec<Instance>, ()> {
    let paths = if let Ok(paths) = fs::read_dir("instances") {
        paths
    } else {
        if fs::create_dir("instances").is_ok() {
            match fs::read_dir("instances") {
                Ok(paths) => paths,
                Err(_) => {
                    println!(
                        "{}",
                        "Failed to read instances folder. Please check again.".red()
                    );
                    return Err(());
                }
            }
        } else {
            println!(
                "{}",
                "Failed to create instances folder. Please check again.".red()
            );
            return Err(());
        }
    };

    let mut instances: Vec<Instance> = vec![];

    for entry in paths {
        if !entry.is_ok() {
            continue;
        }
        let path = entry.unwrap().path().display().to_string();

        if
            let Ok(config) = config_reader::json(
                &format!("{}\\config.mccl.json", path)
            )
        {
            instances.push(Instance { path, config });
        }
    }

    Ok(instances)
}
