use axum::{Json, Router, extract::Path, routing::{get, post}};
use serde::Serialize;

use crate::api::{store_locator::get_nearby_stores, store_stock::{NameLookupParams, StockLookupParams, get_item_name, get_stock}};

mod store_locator;
mod store_stock;

#[derive(Serialize)]
pub struct Response<T>
where T: Serialize
    {
        pub success: bool,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub error_reason: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<T>
    }

impl<T: Serialize> Response<T> {
    pub fn ok(data: T) -> Self {
        Response {
            success: true,
            error_reason: None,
            data: Some(data)
        }
    }

    pub fn err(error_reason: impl Into<String>) -> Self {
        Response {
            success: false,
            error_reason: Some(error_reason.into()),
            data: None
        }
    }
}

pub fn get_api_router() -> Router {
    Router::new()
        .route("/", get(|| async {
            return Json(Response::ok("API is online"))
        }))
        .route("/storelocator/{latitude}/{longitude}", get(
                |Path((latitude, longitude)): Path<(String, String)>| async move {
                    let latitude = match latitude.parse::<f64>() {
                        Ok(value) => value,
                        Err(error) => {
                        eprintln!("Argument parse error: {}", error.to_string());
                            return Json(Response::err(error.to_string()));
                        }
                    };

                    let longitude = match longitude.parse::<f64>() {
                        Ok(value) => value,
                        Err(error) => {
                            eprintln!("Argument parse error: {}", error.to_string());
                            return Json(Response::err(error.to_string()));
                        }
                    };

                    let store_locator = match get_nearby_stores(latitude, longitude).await {
                        Ok(value) => value,
                        Err(error) => {
                            eprintln!("Store locator API failed: {}", error.to_string());
                            return Json(Response::err(error));
                        }
                    };

                    Json(Response::ok(store_locator))
                },
            ),
        )
        .route("/stock", post(|Json(params): Json<StockLookupParams>| async move {
            match get_stock(params).await {
                Ok(stock_result) => Json(Response::ok(stock_result)),
                Err(err) => {
                    eprintln!("Stock lookup failed: {}", err.to_string());
                    Json(Response::err(err))
                }
            }
        }))
        .route("/name", post(|Json(params): Json<NameLookupParams>| async move {
            match get_item_name(params).await {
                Ok(stock_result) => Json(Response::ok(stock_result)),
                Err(err) => {
                    eprintln!("Name lookup failed: {}", err.to_string());
                    Json(Response::err(err))
                }
            }
        }))
}
