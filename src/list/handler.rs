use crate::list::{
    database,
    database_model::ListDatabaseModel,
    request_model::NewListRequestModel
};
use crate::shared::request_model::FilterOptions;

use crate::AppState;
use axum::{
    body::{Body, Bytes},
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
    response::{IntoResponse, Response},
    debug_handler
};
use futures::stream;
use futures::stream::iter;
use http::header::LOCATION;
use mongodb::Collection;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

#[debug_handler]
pub async fn get_lists_handler(

    State(app_state): State<Arc<AppState>>,
    opts: Option<Query<FilterOptions>>,

) -> Response {
    let Query(opts) = opts.unwrap_or_default();
    let limit = opts.limit.unwrap_or(10) as i64;
    let page = opts.page.unwrap_or(1) as i64;

    let collection: Collection<ListDatabaseModel> = app_state.db.collection("TodoList");
    match database::fetch_lists(&collection, limit, page).await {
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
pub async fn create_list_handler(
    State(app_state): State<Arc<AppState>>, 
    Json(body): Json<NewListRequestModel>,
) -> Response {

    let new_list = ListDatabaseModel::new(body.name.clone());

    let collection = app_state.db.collection("TodoList");
    match database::create_list(&collection, &new_list).await {
        Ok(list) => {
            let json_list = list.to_response_body();

            let location = format!("http://{}/lists/{}", app_state.app_host, list._id);
            
            Response::builder()
                .header(LOCATION, location)
                .header(http::header::CONTENT_TYPE, "application/json")
                .status(StatusCode::CREATED)
                .body(json_list)
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
pub async fn get_list_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match app_state
        .collection_item
        .get_item(&id)
        .await
        .map_err(CollectionError::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn edit_list_handler(
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

pub async fn delete_list_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match app_state
        .collection_item
        .delete_item(&id)
        .await
        .map_err(CollectionError::from)
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(e.into()),
    }
}
*/
