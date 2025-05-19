use colored::Colorize;

use crate::{
    functions::connect_node,
    structs::{ instance::Instance, service::ServiceCheck },
    utils::pause,
};

pub async fn mount(instance: &Instance) {
    let client = reqwest::Client::new();

    let request = client
        .get(format!("{}{}", instance.config.service, "/session/check"))
        .header(
            "Authorization",
            format!("{} {}", instance.config.username, instance.config.token)
        )
        .send().await;

    match request {
        Ok(response) => {
            let parsed_response = response.json::<ServiceCheck>().await;
            match parsed_response {
                Ok(check) => {
                    connect_node::establish(check).await;
                }
                Err(message) => {
                    println!(
                        "{}\n{}\n\n{}",
                        "> Can't read check request from the remote service. Error log:"
                            .red()
                            .bold(),
                        message,
                        "Press any key to go back...".red()
                    );
                    pause::invoke();

                    return;
                }
            }
        }
        Err(message) => {
            println!(
                "{}\n{}\n\n{}",
                "> Can't send check request to the remote service, please check your internet connection. Error log:"
                    .red()
                    .bold(),
                message,
                "Press any key to go back...".red()
            );
            pause::invoke();

            return;
        }
    }
}
