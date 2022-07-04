use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Strategy {
    pub id: u32,
    pub accept_id: u32,
    pub access_type: u8,
    pub service_type: u32,
    pub targeting_filter_id: u32,
    pub plan: u32,
    pub code_id: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AdSource {
    pub id: u32,
    pub name: String,
    pub service_type: String,
    pub source_type: String,
    pub url: String,
}