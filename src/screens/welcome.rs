use std::io::{ Error, ErrorKind, Write, stdout };
use crate::{ services::instances_reader, utils::pause };
use colored::Colorize;
use inquire::Select;

pub fn mount() -> Result<String, Error> {
    let instances = if let Ok(instances) = instances_reader::read("instances") {
        instances
    } else {
        print!(
            "{} Can't read the `instances` folder. Maybe the folder doesn't exist?\n\n{}",
            ">".red(),
            "Press any key to exit...".red()
        );
        let _ = stdout().flush();

        pause::invoke();
        return Err(Error::new(ErrorKind::Other, "Can't read `instances`"));
    };

    let mut instances_display: Vec<String> = vec![
        "* Create a new instance".to_string()
    ];
    for instance in instances.iter().enumerate() {
        instances_display.push(
            format!(
                "{}. {} ({})",
                instance.0 + 1,
                instance.1.config.name,
                instance.1.path
            )
        );
    }

    Ok(
        Select::new(
            "Welcome to MCCL Node :3 ! Please select an instance to proceed ->",
            instances_display
        )
            .prompt()
            .unwrap()
    )
}
