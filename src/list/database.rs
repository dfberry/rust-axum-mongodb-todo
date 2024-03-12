
use chrono::prelude::*;
use futures::StreamExt;
use mongodb::bson::{doc, oid::ObjectId, Document};
use mongodb::options::{FindOneAndUpdateOptions, FindOptions, IndexOptions, ReturnDocument};
use mongodb::Database;
use mongodb::{bson, options::ClientOptions, Client, Collection, IndexModel};
use std::str::FromStr;
use std::error::Error;
use crate::list::model::ListModel;
use crate::database_error::MyDBError::MongoQueryError;

pub async fn fetch_list( collection: &Collection<ListModel>, limit: i64, page: i64) -> Result<Vec<ListModel>, Box<dyn Error>> {
    let find_options = FindOptions::builder()
        .limit(limit)
        .skip(u64::try_from((page - 1) * limit).unwrap())
        .build();

    let mut cursor = collection
        .find(None, find_options)
        .await
        .map_err(MongoQueryError)?;

    let mut db_result: Vec<ListModel> = Vec::new();
    while let Some(doc) = cursor.next().await {
        match doc {
            Ok(item) => db_result.push(item),
            Err(e) => {
                println!("Error processing document: {}", e);
                continue;
            },
        }
    }

    println!("fetch_notes returns {:?}", db_result);

    Ok(db_result)
}
/*
    pub async fn create_list(&self, body: &CreateListSchema) -> Result<SingleListResponse> {

        let document = self.create_list_document(body)?;

        let options = IndexOptions::builder().unique(true).build();
        let index = IndexModel::builder()
            .keys(doc! {"name": 1})
            .options(options)
            .build();

        match self.collection_client_with_type.create_index(index, None).await {
            Ok(_) => {}
            Err(e) => return Err(MongoQueryError(e)),
        };

        let insert_result = match self.collection_client_without_type.insert_one(&document, None).await {
            Ok(result) => result,
            Err(e) => {
                if e.to_string()
                    .contains("E11000 duplicate key error collection")
                {
                    return Err(MongoDuplicateError(e));
                }
                return Err(MongoQueryError(e));
            }
        };

        let new_id = insert_result
            .inserted_id
            .as_object_id()
            .expect("issue with new _id");

        let list_doc = match self
            .collection_client_with_type
            .find_one(doc! {"_id": new_id}, None)
            .await
        {
            Ok(Some(doc)) => doc,
            Ok(None) => return Err(NotFoundError(new_id.to_string())),
            Err(e) => return Err(MongoQueryError(e)),
        };

        Ok(SingleListResponse {
            status: "success",
            data: ListData {
                list: self.doc_to_list(&list_doc)?,
            },
        })
    }

    pub async fn get_list(&self, id: &str) -> Result<SingleListResponse> {
        let oid = ObjectId::from_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;

        let list_doc = self
            .collection_client_with_type
            .find_one(doc! {"_id":oid }, None)
            .await
            .map_err(MongoQueryError)?;

        match list_doc {
            Some(doc) => {
                let list = self.doc_to_list(&doc)?;
                Ok(SingleListResponse {
                    status: "success",
                    data: ListData { list },
                })
            }
            None => Err(NotFoundError(id.to_string())),
        }
    }

    pub async fn edit_list(&self, id: &str, body: &UpdateListSchema) -> Result<SingleListResponse> {
        let oid = ObjectId::from_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;

        let update = doc! {
            "$set": bson::to_document(body).map_err(MongoSerializeBsonError)?,
        };

        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        if let Some(doc) = self
            .collection_client_with_type
            .find_one_and_update(doc! {"_id": oid}, update, options)
            .await
            .map_err(MongoQueryError)?
        {
            let list = self.doc_to_list(&doc)?;
            let list_response = SingleListResponse {
                status: "success",
                data: ListData { list },
            };
            Ok(list_response)
        } else {
            Err(NotFoundError(id.to_string()))
        }
    }

    pub async fn delete_list(&self, id: &str) -> Result<()> {
        let oid = ObjectId::from_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let filter = doc! {"_id": oid };

        let result = self
            .collection_client_without_type
            .delete_one(filter, None)
            .await
            .map_err(MongoQueryError)?;

        match result.deleted_count {
            0 => Err(NotFoundError(id.to_string())),
            _ => Ok(()),
        }
    }

    fn doc_to_list(&self, list: &ListModel) -> Result<ListResponse> {

        println!("doc_to_list::list: {:?}", list);

        let list_response = ListResponse {
            id: list.id.to_hex(),
            name: list.name.to_owned(),
            createdAt: list.createdAt,
            updatedAt: list.updatedAt,
        };

        Ok(list_response)
    }

    fn create_list_document(
        &self,
        body: &CreateListSchema
    ) -> Result<bson::Document> {
        let serialized_data = bson::to_bson(body).map_err(MongoSerializeBsonError)?;
        let document = serialized_data.as_document().unwrap();

        let datetime = Utc::now();

        let mut doc_with_dates = doc! {
            "createdAt": datetime,
            "updatedAt": datetime
        };
        doc_with_dates.extend(document.clone());

        Ok(doc_with_dates)
    }
}
*/
