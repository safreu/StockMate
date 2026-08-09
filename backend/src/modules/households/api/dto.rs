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

#[derive(Debug, Deserialize)]
pub struct AddHouseholdMemberRequest {
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct HouseholdMemberResponse {
    pub user_id: String,
    pub display_name: String,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct RenameHouseholdRequest {
    pub name: String,
}
