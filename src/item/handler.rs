use std::sync::Arc;

use axum::{
    body::{Body, Bytes},
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
    response::{IntoResponse, Response},
};
use futures::stream;
use futures::stream::iter;
use http::header::LOCATION;
use mongodb::Collection;
use serde::Deserialize;
use serde_json::json;
use axum::debug_handler;
use crate::{
    AppState,
    shared::request_model::FilterOptions,
};
use crate::item::{
    database_model::ItemDatabaseModel,
    database,
    request_model::NewItemRequestModel
};

#[debug_handler]
pub async fn get_items_handler(
    State(app_state): State<Arc<AppState>>,
    Path(listId): Path<String>,
    opts: Option<Query<FilterOptions>>,
) -> Response {
    let Query(opts) = opts.unwrap_or_default();
    let limit = opts.limit.unwrap_or(10) as i64;
    let page = opts.page.unwrap_or(1) as i64;

    let collection: Collection<ItemDatabaseModel> = app_state.db.collection("TodoItem");

    match database::fetch_items(&collection, &listId, limit, page).await {
        Ok(res) => {
            let res: Vec<_> = res.iter().map(|x| x.read()).collect();

            // convert res to Body
            let body = Body::from(serde_json::to_string(&res).unwrap());

            Response::builder()
            .header(http::header::CONTENT_TYPE, "application/json")
            .status(StatusCode::OK)
            .body(body)
            .unwrap()
        }
        Err(e) => {

            let error_message = json!({ "error": e.to_string() });
            let error_body = Body::from(error_message.to_string());

            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error_body)
                .unwrap()
        },
    }
    
}

#[debug_handler]
pub async fn create_item_handler(
    State(app_state): State<Arc<AppState>>, 
    Json(body): Json<NewItemRequestModel>,
) -> Response {

    let new_item = ItemDatabaseModel::new(
        body.listId.clone(),
        body.name.clone(),
        body.state.clone().unwrap(),
        body.description.clone(),
        body.dueDate.clone(),

    );
    let collection = app_state.db.collection("TodoItem");

    match database::create_item(&collection, &new_item).await {
        Ok(item) => {
            let json_item = item.to_response_body();

            let location = format!("http://{}/lists/{}/{}", app_state.app_host, new_item.listId, item._id, );
            
            Response::builder()
                .header(LOCATION, location)
                .header(http::header::CONTENT_TYPE, "application/json")
                .status(StatusCode::CREATED)
                .body(json_item)
                .unwrap()

        }
        Err(e) => {

            let error_message = json!({ "error": e.to_string() });
            let error_body = Body::from(error_message.to_string());

            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error_body)
                .unwrap()
        },
    }
}
/* 
pub async fn get_item_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match app_state.collection_item.get_item(&id).await.map_err(CollectionError::from) {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn edit_item_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<UpdateItemSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match app_state
        .collection_item
        .edit_item(&id, &body)
        .await
        .map_err(CollectionError::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn delete_item_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match app_state.collection_item.delete_item(&id).await.map_err(CollectionError::from) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(e.into()),
    }
}

// pub async fn get_items_state_handler(
//     Path(list_id): Path<String>,
//     Path(state): Path<String>,
//     State(app_state): State<Arc<AppState>>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     let skip = app_state.query.skip.unwrap_or(0);
//     let limit = app_state.query.top.unwrap_or(20);

//     match app_state.collection_item.get_items_state(&list_id, &state, skip, limit).await.map_err(CollectionError::from) {
//         Ok(res) => Ok(Json(res)),
//         Err(e) => Err(e.into()),
//     }
// }
// pub async fn edit_list_items_state_handler(
//     Path(list_id): Path<String>,
//     Path(state): Path<String>,
//     State(app_state): State<Arc<AppState>>,
//     Json(body): Json<UpdateItemSchema>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     match app_state.collection_item.edit_list_items_state(&list_id, &state, &body).await.map_err(CollectionError::from) {
//         Ok(res) => Ok(Json(res)),
//         Err(e) => Err(e.into()),
//     }
// }

*/