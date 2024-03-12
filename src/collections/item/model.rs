// https://stackoverflow.com/questions/55735974/how-do-i-serialize-chronodatetime-fields-as-isodate-when-using-the-rust-mongo
// https://stackoverflow.com/questions/67803619/using-serdeserialize-with-optionchronodatetime
use chrono::prelude::*;
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, Map};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub listId: ObjectId,
    pub name: String,
    pub state: String,
    pub description: Option<String>,
    pub dueDate: Option<DateTime<Utc>>,
    pub completedDate: Option<DateTime<Utc>>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updatedAt: DateTime<Utc>,
}