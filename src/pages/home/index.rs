use colored::Colorize;

use crate::{
    functions::instances,
    pages,
    structs::instance::Instance,
    utils::pause,
};

pub async fn mount() {
    let mut options: Vec<String> = vec![];

    let instances_fetch: Vec<Instance>;
    loop {
        instances_fetch = if let Ok(cache) = instances::fetch() {
            cache
        } else {
            println!(
                "The program ran into a problem with instances reading. Press any key to try again..."
            );
            pause::invoke();
            continue;
        };

        break;
    }

    for instance in instances_fetch.iter().enumerate() {
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

    let selected_option = ask.prompt().unwrap();

    if selected_option.starts_with("*") {
        // New instance
    } else {
        // Split off some of the prefix characters from styling the text.
        let index =
            selected_option
                .split(".")
                .next()
                .unwrap()[4..]
                .parse::<usize>()
                .unwrap() - 1;

        let selected_instance = instances_fetch.get(index).unwrap();

        pages::instance::index::mount(selected_instance).await;
    }
}
