use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    collections::error::CollectionError,
    collections::model::FilterOptions,
    collections::item::schema::{CreateItemSchema, UpdateItemSchema},
    AppState,
};

pub async fn list_item_handler(
    opts: Option<Query<FilterOptions>>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10) as i64;
    let page = opts.page.unwrap_or(1) as i64;

    match app_state
        .collection_item
        .fetch_items(limit, page)
        .await
        .map_err(CollectionError::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}


pub async fn create_item_handler(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<CreateItemSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match app_state.collection_item.create_item(&body).await.map_err(CollectionError::from) {
        Ok(res) => Ok((StatusCode::CREATED, Json(res))),
        Err(e) => Err(e.into()),
    }
}

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

