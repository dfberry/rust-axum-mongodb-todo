

use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewListRequestModel {
    pub name: String

}

#[derive(Deserialize)]
pub struct UpdateListRequestModel {
    pub id: Option<String>,
    pub name: String,
    pub createdAt: String,
    pub updatedDate: Option<String>

}