mod http_block_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    http_block_server::run_announce_server("_name_".to_string(), 25566).await
}
