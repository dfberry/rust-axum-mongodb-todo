use std::sync::Arc;

use axum::{
    routing::{get, post, patch, delete},
    Router,
};

use crate::list::handler::{
    get_lists_handler,
    // create_list_handler,
    // get_list_handler,
    // edit_list_handler,
    // delete_list_handler,
    // create_item_handler,
    // get_item_handler,
    // edit_item_handler,
    // delete_item_handler,
    // get_items_state_handler,
    // edit_list_items_state_handler,
};

use crate::AppState;
use tower::ServiceBuilder;
use tower::layer::Layer;
use tracing::Level;
use tower_http::{
    LatencyUnit,
    trace::{
        TraceLayer, DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse}};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/lists", list_routes(app_state.clone()))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new().include_headers(true)
                )
                .on_request(
                    DefaultOnRequest::new().level(Level::INFO)
                )
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Micros)
                ))
                //.layer(Extension(State {}))
        )
        //.nest("/lists/:listId/items", list_items_routes(app_state.clone()))
}

pub fn list_routes(app_state: Arc<AppState>) -> Router {
    Router::new()//.route("/", post(create_list_handler))
    .route("/", get(get_lists_handler))
//     .route(
//        "/:id",
//        get(get_list_handler)
//            .put(edit_list_handler)
//            .delete(delete_list_handler)
//    )
   .with_state(app_state.clone())
}
// pub fn list_items_routes(app_state: Arc<AppState>) -> Router {
//     Router::new().route("/", post(create_item_handler))
//     .with_state(app_state.clone())
// //    .route("/", get(list_item_handler))
// //     .route(
// //        "/:id",
// //        get(get_item_handler)
// //            .put(edit_item_handler)
// //            .delete(delete_item_handler)
// //    )
// //    .route("/state/:state", 
// //         get(get_items_state_handler)
// //         .put(edit_list_items_state_handler)).with_state(app_state.clone())
// }