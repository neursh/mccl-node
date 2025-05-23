use colored::Colorize;
use serde_json::json;

use crate::{
    structs::{ instance::Instance, service::ServiceCheck },
    utils::pause,
};

pub async fn request(
    instance: &Instance,
    nodeid: &Vec<u8>,
    alpn: &Vec<u8>
) -> Result<(), ()> {
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
                    if check.status == "running" {
                        println!(
                            "{} {} {}\n\n{}",
                            ">".red().bold(),
                            check.host.unwrap().red().bold(),
                            "is already hosting this instance!".red().bold(),
                            "Press any key to go back...".red()
                        );

                        return Err(());
                    }
                }
                Err(message) => {
                    println!(
                        "{}\n{}\n\n{}",
                        "> Can't read response from the remote service. Error log:"
                            .red()
                            .bold(),
                        message,
                        "Press any key to go back...".red()
                    );
                    pause::invoke();

                    return Err(());
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

            return Err(());
        }
    }

    println!(
        "{} No active session found! Hosting this session...",
        ">".green()
    );

    let request = client
        .post(format!("{}{}", instance.config.service, "/session/start"))
        .header(
            "Authorization",
            format!("{} {}", instance.config.username, instance.config.token)
        )
        .json(
            &json!({
            "nodeid": nodeid,
            "alpn": alpn
        })
        )
        .send().await;

    match request {
        Ok(response) => {
            let parsed_response = response.json::<ServiceCheck>().await;
            match parsed_response {
                Ok(check) => {
                    if check.status == "running" {
                        println!(
                            "{} {} {}\n\n{}",
                            ">".red().bold(),
                            check.host.unwrap().red().bold(),
                            "is already hosting this instance while we are registering for it!"
                                .red()
                                .bold(),
                            "Press any key to go back...".red()
                        );

                        return Err(());
                    }
                }
                Err(message) => {
                    println!(
                        "{}\n{}\n\n{}",
                        "> Can't read response from the remote service. Error log:"
                            .red()
                            .bold(),
                        message,
                        "Press any key to go back...".red()
                    );
                    pause::invoke();

                    return Err(());
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

            return Err(());
        }
    }

    return Ok(());
}
