use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstanceConfig {
    pub name: String,
    pub username: String,
    pub token: String,
    pub service: String,
    pub discord_webhook: Option<String>,
    pub java_runtime: String,
    pub args: Vec<String>,
    pub untracked: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Instance {
    pub path: String,
    pub config: InstanceConfig,
}
