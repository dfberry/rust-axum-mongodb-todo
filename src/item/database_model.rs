use mongodb::bson::doc;
use mongodb::bson::DateTime;
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use bson::Bson;
use bson::DateTime as BsonDateTime;
use std::str::FromStr;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TodoItemState {
    #[serde(rename = "todo")]
    Todo,
    #[serde(rename = "inprogress")]
    InProgress,
    #[serde(rename = "done")]
    Done,
}
impl TodoItemState {
    pub fn to_str(state: &Self) -> &'static str {
        match *state {
            TodoItemState::Todo => "todo",
            TodoItemState::InProgress => "inprogress",
            TodoItemState::Done => "done",
        }
    }
    pub fn from_string(s: &str) -> Option<Self> {

        println!("state from_string: {:?}", s);

        match s {
            "todo" => Some(TodoItemState::Todo),
            "inprogress" => Some(TodoItemState::InProgress),
            "done" => Some(TodoItemState::Done),
            _ => Some(TodoItemState::Todo),
        }
    }
}
#[allow(non_snake_case)]
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemDatabaseModel {
    pub _id: ObjectId,
    pub listId: ObjectId,
    pub name: String,
    pub state: Option<TodoItemState>,
    pub description: Option<String>,
    pub dueDate: Option<DateTime>,
    pub completedDate: Option<DateTime>,
    pub createdDate: DateTime,
    pub updatedDate: Option<DateTime>,
}
impl ItemDatabaseModel {
    pub fn string_to_optional_datetime2(s: &String) -> Option<BsonDateTime> {
        match BsonDateTime::parse_rfc3339_str(s) {
          Ok(dt) => Some(dt.into()),
          Err(_) => None, // handle error as you see fit
        }
      }
      pub fn string_to_datetime(s: &String) -> BsonDateTime {
        let new_object = BsonDateTime::parse_rfc3339_str(s).unwrap();
        return new_object
      }
    pub fn new(
        listId: String,
        name: String,
        state: String, 
        description: Option<String>,
        dueDate: Option<String>,
    ) -> Self {

        let now = bson::DateTime::now();
        let due_date = match dueDate {
            Some(s) => ItemDatabaseModel::string_to_optional_datetime2(&s),
            None => None
        };

        let state = TodoItemState::from_string(&state);

        Self {
            _id: ObjectId::new(),
            listId: ObjectId::from_str(&listId).unwrap(),
            name: name,
            state: state,
            description: description,
            dueDate: due_date,
            completedDate: None,
            createdDate: now,
            updatedDate: None,

        }
    }
    pub async fn update(
        id: String, 
        listid: String,
        name: String,
        state: Option<String>,
        description: Option<String>,
        dueDate: Option<String>,
        completedDate: Option<String>,
        createdDate: String,
        updatedDated: Option<String>
    ) -> Self {
        let now = bson::DateTime::now();
        let due_date = match dueDate {
            Some(s) => ItemDatabaseModel::string_to_optional_datetime2(&s),
            None => None
        };
        let completed_date = match completedDate {
            Some(s) => ItemDatabaseModel::string_to_optional_datetime2(&s),
            None => None
        };
        let created_date = ItemDatabaseModel::string_to_datetime(&createdDate);

        let state = state.unwrap();
        let optional_state = TodoItemState::from_string(&state);

        Self {
            _id: ObjectId::from_str(&id).unwrap(),
            listId: ObjectId::from_str(&listid).unwrap(),
            name: name,
            state: optional_state,
            description: description,
            dueDate: due_date,
            completedDate: completed_date,
            createdDate: created_date,
            updatedDate: Some(now),
        }
    }
    pub fn read(&self) -> Bson {
        // convert _id and listId from ObjectId to string
        let id = self._id.to_hex();
        let list_id = self.listId.to_hex();
        let name = self.name.clone();

        let state_obj: TodoItemState = self.state.clone().unwrap();
        let state = TodoItemState::to_str(&state_obj);

        let description = self.description.clone();

        let due_date = self.dueDate.map(|date| date.try_to_rfc3339_string().unwrap());
        let completed_date = self.completedDate.map(|date| date.try_to_rfc3339_string().unwrap());
        let updated_date = self.updatedDate.map(|date| date.try_to_rfc3339_string().unwrap());
    
        let created_date = self.createdDate.try_to_rfc3339_string().unwrap();
    
        let doc = doc! {
            "id": id,
            "listId": list_id,
            "name": name,
            "state": state,
            "description": description,
            "dueDate": due_date,
            "completedDate": completed_date,
            "createdDate": created_date,
            "updatedDate": updated_date,
        };

        Bson::Document(doc)
    }
    pub fn to_response_body(&self) -> axum::body::Body {
        let body = self.read();
        axum::body::Body::from(serde_json::to_string(&body).unwrap())
    }
}
