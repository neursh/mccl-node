use std::{ io::{ stdout, Write }, path::{ Path, PathBuf } };
use colored::Colorize;
use inquire::{ Confirm, Select, Text };
use crate::{
    services::create_instance,
    structs::instance::Instance,
    utils::{ clear, pause },
};

pub fn mount(instance: Instance) {
    let selected_option = Select::new(
        &format!(
            "Instance {} | What would you like to do?",
            instance.config.name.bright_cyan()
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
}
