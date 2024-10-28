mod utils;
mod services;
mod instance;
mod screens;

use utils::clear;

#[rocket::main]
async fn main() -> std::io::Result<()> {
    loop {
        clear::invoke();

        let selected_instance = match screens::welcome::mount() {
            Ok(selected) => selected,
            Err(err) => {
                return Err(err);
            }
        };

        if selected_instance == "* Create a new instance" {
            screens::new_instance::mount();
        }
    }

    Ok(())
}
