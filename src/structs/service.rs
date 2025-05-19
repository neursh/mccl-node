use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServiceCheck {
    pub status: bool,
    pub host: Option<String>,
    pub last_run: Option<u64>,
    pub nodeid: Option<Vec<u8>>,
    pub alpn: Option<Vec<u8>>,
}
