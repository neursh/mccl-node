use std::io::{ Error, ErrorKind, Write, stdout };
use crate::{
    services::instances_reader,
    structs::instance::Instance,
    utils::pause,
};
use colored::Colorize;
use inquire::Select;

pub struct ResponseInfo {
    pub text: String,
    pub instance: Option<Instance>,
}

pub fn mount() -> Result<ResponseInfo, Error> {
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

    let selected = Select::new(
        "Welcome to MCCL Node :3 | Please select an instance to proceed ->",
        instances_display
    )
        .prompt()
        .unwrap();
    let selected_instance: Option<Instance> = if
        let Some(number) = selected.split(". ").next()
    {
        if let Ok(parsed_number) = number.parse::<usize>() {
            Some(instances[parsed_number - 1].clone())
        } else {
            None
        }
    } else {
        None
    };

    Ok(ResponseInfo { text: selected, instance: selected_instance })
}
