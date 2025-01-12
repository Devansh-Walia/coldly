use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ContactRecord {
    #[serde(rename = "Email address")]
    pub email_address: Option<String>,
    #[serde(rename = "Domain name")]
    pub domain_name: Option<String>,
    pub organization: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    #[serde(rename = "Postal code")]
    pub postal_code: Option<String>,
    pub street: Option<String>,
    #[serde(rename = "Confidence score")]
    pub confidence_score: Option<String>,
    #[serde(rename = "Type")]
    pub record_type: Option<String>,
    #[serde(rename = "Number of sources")]
    pub number_of_sources: Option<String>,
    pub pattern: Option<String>,
    #[serde(rename = "First name")]
    pub first_name: Option<String>,
    #[serde(rename = "Last name")]
    pub last_name: Option<String>,
    pub department: Option<String>,
    pub position: Option<String>,
    #[serde(rename = "Twitter handle")]
    pub twitter_handle: Option<String>,
    #[serde(rename = "LinkedIn URL")]
    pub linkedin_url: Option<String>,
    #[serde(rename = "Phone number")]
    pub phone_number: Option<String>,
    #[serde(rename = "Company type")]
    pub company_type: Option<String>,
    pub industry: Option<String>,
} 