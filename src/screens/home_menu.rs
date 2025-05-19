use colored::Colorize;

use crate::structs::instance::Instance;

pub fn mount(instances: &Vec<Instance>) -> String {
    let mut options: Vec<String> = vec![];

    for instance in instances.iter().enumerate() {
        options.push(
            format!(
                "{}",
                format!(
                    "{}. {} ({})",
                    instance.0 + 1,
                    instance.1.config.name,
                    instance.1.path
                ).bold()
            )
        );
    }

    options.push("*  Create a new instance".to_owned());

    let ask = inquire::Select::new(
        "Please select an instance to proceed >",
        options
    );

    ask.prompt().unwrap()
}
