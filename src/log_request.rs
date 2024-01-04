use crate::ctx::Ctx;
use crate::error::ClientError;
use crate::error::{Error, Result};
use crate::middle_ware::req_stamp::ReqStamp;
use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;

pub async fn log_req(
    req_method: Method,
    uri: Uri,
    ctx: Option<Ctx>,
    req_stamp: ReqStamp,
    service_error: Option<&Error>,
    client_error: Option<ClientError>,
) -> Result<()> {
    let error_type = service_error.map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(service_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    let ReqStamp { uuid, time_in } = req_stamp;

    // Create the RequestLogLine
    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: time_in.to_string(),

        req_path: uri.to_string(),
        req_method: req_method.to_string(),

        user_id: ctx.map(|c| c.user_id()),

        client_error_type: client_error.map(|e| e.as_ref().to_string()),

        error_type,
        error_data,
    };

    println!("   ->> log_request: \n{}", json!(log_line));

    Ok(())
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,      // uuid string formatted
    timestamp: String, // (should be iso8601)

    // -- User and context attributes.
    user_id: Option<u64>,

    // -- http request attributes.
    req_path: String,
    req_method: String,

    // -- Errors attributes.
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}
