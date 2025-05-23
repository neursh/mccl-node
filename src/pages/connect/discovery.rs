use colored::Colorize;

use crate::structs::{ instance::InstanceConfig, service::ServiceCheck };

pub async fn find(config: &InstanceConfig) -> Result<ServiceCheck, ()> {
    let client = reqwest::Client::new();

    let request = client
        .get(format!("{}{}", config.service, "/session/check"))
        .header(
            "Authorization",
            format!("{} {}", config.username, config.token)
        )
        .send().await;

    match request {
        Ok(response) => {
            let parsed_response = response.json::<ServiceCheck>().await;
            match parsed_response {
                Ok(check) => {
                    return Ok(check);
                }
                Err(message) => {
                    println!(
                        "{}\n{}",
                        "> Can't read check request from the remote service. Error log:"
                            .red()
                            .bold(),
                        message
                    );
                    return Err(());
                }
            }
        }
        Err(message) => {
            println!(
                "{}\n{}",
                "> Can't send check request to the remote service, please check your internet connection. Error log:"
                    .red()
                    .bold(),
                message
            );

            return Err(());
        }
    }
}
