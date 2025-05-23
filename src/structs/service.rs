use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServiceCheck {
    pub status: String,
    pub host: Option<String>,
    pub nodeid: Option<Vec<u8>>,
    pub alpn: Option<Vec<u8>>,
}
