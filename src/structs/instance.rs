use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
pub struct InstanceConfig {
    pub name: String,
    pub username: String,
    pub token: String,
    pub service: Option<String>,
    pub discord_webhook: Option<String>,
    pub local_last_run: u64,
    pub executable: String,
    pub cmd: Vec<String>,
    pub excluded_lock_structure: Vec<String>,
}

pub struct Instance {
    pub path: String,
    pub config: InstanceConfig,
}
