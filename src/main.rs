mod utils;
mod services;
mod screens;
mod structs;

use utils::clear;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    loop {
        clear::invoke();

        let selected_instance = match screens::welcome::mount() {
            Ok(selected) => selected,
            Err(err) => {
                return Err(err);
            }
        };

        if selected_instance.text == "*  Create a new instance" {
            screens::new_instance::mount();
        } else {
            screens::instance_menu::mount(selected_instance.instance.unwrap());
        }
    }

    Ok(())
}
