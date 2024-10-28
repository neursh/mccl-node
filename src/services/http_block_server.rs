use rocket::State;

struct NameState {
    name: &'static str,
}

#[rocket::get("/")]
fn world(name_state: &State<NameState>) -> &str {
    name_state.name
}

pub async fn run(name: &'static str, port: u16) {
    let name_state = NameState { name };

    let _ = rocket
        ::build()
        .manage(name_state)
        .configure(
            rocket::Config
                ::figment()
                .merge(("port", port))
                .merge(("log_level", "off"))
        )
        .mount("/", rocket::routes![world])
        .launch().await;
}
