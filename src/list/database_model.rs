use mongodb::bson::doc;
use mongodb::bson::DateTime;
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use bson::Bson;
use bson::DateTime as BsonDateTime;
use std::str::FromStr;

#[allow(non_snake_case)]
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListDatabaseModel {
    pub _id: ObjectId,
    pub name: String,
    pub createdDate: DateTime,
    pub updatedDate: DateTime,

}
impl ListDatabaseModel {
    pub fn new(name: String) -> Self {
        let now = bson::DateTime::now();
        Self {
            _id: ObjectId::new(),
            name: name,
            createdDate: now,
            updatedDate: now,

        }
    }
    pub fn update(id: String, name: String, createdAt: String) -> Self {

        println!("ListDatabaseModel::update - id: {:?}", id);

        // change string into DateTime
        let createdDate = BsonDateTime::parse_rfc3339_str(&createdAt).unwrap();

        Self {
            _id: ObjectId::from_str(&id).unwrap(),
            name: name,
            createdDate: createdDate,
            updatedDate: bson::DateTime::now(),
        }
    }
    pub fn read(&self) -> Bson {
        // convert _id from ObjectId to string
        let id = self._id.to_hex();

        let createdDate = self.createdDate.try_to_rfc3339_string().unwrap();
        let updatedDate = self.updatedDate.try_to_rfc3339_string().unwrap();

        let doc = doc! {
            "id": id,
            "name": self.name.clone(),
            "createdAt": createdDate,
            "updatedAt": updatedDate,
        };

        Bson::Document(doc)
    }
    pub fn to_response_body(&self) -> axum::body::Body {
        let body = self.read();
        axum::body::Body::from(serde_json::to_string(&body).unwrap())
    }
}
