use colored::Colorize;

use crate::{ structs::instance::Instance, utils::pause };

use super::{ connect_node, discovery };

pub async fn mount(instance: &Instance) {
    if let Ok(check) = discovery::find(&instance.config).await {
        connect_node::establish(check).await;
    } else {
        println!("\n{}", "Press any key to go back...".red());
        pause::invoke();

        return;
    }
}
