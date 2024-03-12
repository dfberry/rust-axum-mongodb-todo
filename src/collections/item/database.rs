use crate::collections::error::CollectionError;

use crate::{
    collections::error::CollectionError::*
};
use crate::collections::item::schema::{CreateItemSchema, UpdateItemSchema};
use crate::collections::item::model::ItemModel;
use crate::handlers::response_item::{ItemData, ItemListResponse, ItemResponse, SingleItemResponse};
use chrono::prelude::*;
use futures::StreamExt;
use mongodb::bson::{doc, oid::ObjectId, Document};
use mongodb::options::{FindOneAndUpdateOptions, FindOptions, IndexOptions, ReturnDocument};
use mongodb::{bson, options::ClientOptions, Client, Collection, IndexModel};
use std::str::FromStr;

use mongodb::bson::{doc, oid::ObjectId, Document};
use mongodb::options::{FindOneAndUpdateOptions, FindOptions, IndexOptions, ReturnDocument};
use mongodb::{bson, options::ClientOptions, Client, Collection, IndexModel};

#[derive(Clone, Debug)]
pub struct CollectionItem {
    pub collection_client_with_type: Collection<ItemModel>,
    pub collection_client_without_type: Collection<Document>,
    pub database: mongodb::Database,
}
type Result<T> = std::result::Result<T, CollectionError>;

impl CollectionItem {
    pub async fn init() -> Result<Self> {
        let mongodb_uri = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let database_name =
            std::env::var("MONGO_INITDB_DATABASE").expect("MONGO_INITDB_DATABASE must be set.");
        let mut client_options = ClientOptions::parse(mongodb_uri).await?;
        client_options.app_name = Some(database_name.to_string());

        let client = Client::with_options(client_options)?;
        let database = client.database(database_name.as_str());

        let collection_client_with_type = database.collection("TodoItem");
        let collection_client_without_type = database.collection::<Document>("TodoItem");

        println!("✅ Database connected successfully");

        Ok(Self {
            collection_client_with_type,
            collection_client_without_type,
            database
        })
    }
    pub async fn fetch_items(&self, limit: i64, page: i64) -> Result<ItemListResponse> {
        let find_options = FindOptions::builder()
            .limit(limit)
            .skip(u64::try_from((page - 1) * limit).unwrap())
            .build();

        let mut cursor = self
            .collection_client_with_type
            .find(None, find_options)
            .await
            .map_err(MongoQueryError)?;

        let mut json_result: Vec<ItemResponse> = Vec::new();
        while let Some(doc) = cursor.next().await {
            json_result.push(self.doc_to_item(&doc.unwrap())?);
        }

        Ok(ItemListResponse {
            status: "success",
            results: json_result.len(),
            items: json_result,
        })
    }

    pub async fn create_item(&self, body: &CreateItemSchema) -> Result<SingleItemResponse> {

        let document = self.create_item_document(body)?;

        let options = IndexOptions::builder().unique(true).build();
        let index = IndexModel::builder()
            .keys(doc! {"name": 1, "listId": 1})
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

        let item_doc = match self
            .collection_client_with_type
            .find_one(doc! {"_id": new_id}, None)
            .await
        {
            Ok(Some(doc)) => doc,
            Ok(None) => return Err(NotFoundError(new_id.to_string())),
            Err(e) => return Err(MongoQueryError(e)),
        };

        Ok(SingleItemResponse {
            status: "success",
            data: ItemData {
                item: self.doc_to_item(&item_doc)?,
            },
        })
    }

    pub async fn get_item(&self, id: &str) -> Result<SingleItemResponse> {
        let oid = ObjectId::from_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;

        let item_doc = self
            .collection_client_with_type
            .find_one(doc! {"_id":oid }, None)
            .await
            .map_err(MongoQueryError)?;

        match item_doc {
            Some(doc) => {
                let item = self.doc_to_item(&doc)?;
                Ok(SingleItemResponse {
                    status: "success",
                    data: ItemData { item },
                })
            }
            None => Err(NotFoundError(id.to_string())),
        }
    }

    pub async fn edit_item(&self, id: &str, body: &UpdateItemSchema) -> Result<SingleItemResponse> {
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
            let item = self.doc_to_item(&doc)?;
            let item_response = SingleItemResponse {
                status: "success",
                data: ItemData { item },
            };
            Ok(item_response)
        } else {
            Err(NotFoundError(id.to_string()))
        }
    }

    pub async fn delete_item(&self, id: &str) -> Result<()> {
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

    fn doc_to_item(&self, item: &ItemModel) -> Result<ItemResponse> {
        
        let dueDate = match item.dueDate {
            Some(date) => Some(date),
            None => None
        };

        let completedDate = match item.completedDate {
            Some(date) => Some(date),
            None => None
        };
        
        let item_response = ItemResponse {
            id: item.id.to_hex(),
            listId: item.listId.to_hex(),
            name: item.name.to_owned(),
            state: item.state.to_owned(),
            description: item.description.to_owned(),
            dueDate:  dueDate,
            completedDate: completedDate
        };

        Ok(item_response)
    }

    fn create_item_document(
        &self,
        body: &CreateItemSchema
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
    // fn get_items_state_handler(&self,
    //     listid, 
    //     state, 
    //     skip, limit
    // ) -> Result {



/*
    const query = TodoItemModel.find({ listId: req.params.listId, state: req.params.state });
    const skip = req.query.skip ? parseInt(req.query.skip) : 0;
    const top = req.query.top ? parseInt(req.query.top) : 20;

    const lists = await query
        .skip(skip)
        .limit(top)
        .exec();

    res.json(lists);
*/

    // }
    // fn edit_list_items_state_handler() -> Result {
/*
    try {
        const completedDate = req.params.state === TodoItemState.Done ? new Date() : undefined;

        const updateTasks = req.body.map(
            id => TodoItemModel
                .findOneAndUpdate(
                    { listId: req.params.listId, _id: id },
                    { state: req.params.state, completedDate: completedDate })
                .orFail()
                .exec()
        );

        await Promise.all(updateTasks);

        res.status(204).send();
    }
    catch (err: any) {
        switch (err.constructor) {
        case mongoose.Error.CastError:
        case mongoose.Error.DocumentNotFoundError:
            return res.status(404).send();
        default:
            throw err;
        }
    }

*/
    //}
}
