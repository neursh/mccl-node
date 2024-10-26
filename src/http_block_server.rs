use actix_web::{ web, App, HttpResponse, HttpServer, Responder };

async fn server_announce(name: String) -> impl Responder {
    HttpResponse::Ok().body(name)
}


pub async fn run_announce_server(
    name: String,
    port: u16
) -> std::io::Result<()> {
    HttpServer::new(move || {
        let name_clone = name.clone();

        App::new().route(
            "/",
            web::get().to(move || server_announce(name_clone.clone()))
        )
    })
        .bind(("127.0.0.1", port))?
        .run().await
}
