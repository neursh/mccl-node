mod http_block_server;
mod server_launcher;

#[rocket::main]
async fn main() -> std::io::Result<()> {
    tokio::spawn(async move {
        match server_launcher::start().await {
            Ok(_) => {},
            Err(_) => println!(""),
        }
    });

    http_block_server::run("_name_", 25566).await;

    Ok(())
}
