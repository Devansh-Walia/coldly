use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ContactRecord {
    #[serde(rename = "Email address")]
    pub email: String,

    #[serde(rename = "Domain name")]
    pub domain_name: Option<String>,
    
    #[serde(rename = "First name")]
    pub first_name: String,
    
    #[serde(rename = "Last name")]
    pub last_name: Option<String>,
} 