use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateHouseholdRequest {
    pub name: String,
    pub kind: String,
}

#[derive(Debug, Serialize)]
pub struct CreateHouseholdResponse {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct HouseholdResponse {
    pub id: String,
    pub name: String,
    pub kind: String,
}
