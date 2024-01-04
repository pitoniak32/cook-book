use crate::{
    error::{Error, Result},
    util::now_utc,
};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::{async_trait, body::Body};
use time::OffsetDateTime;
use tracing::debug;
use uuid::Uuid;

pub async fn request_stamp(mut req: Request<Body>, next: Next) -> Result<Response> {
    debug!("{:<12} - mw_req_stamp_resolver", "MIDDLEWARE");

    let time_in = now_utc();
    let uuid = Uuid::new_v4();

    req.extensions_mut().insert(ReqStamp { uuid, time_in });

    Ok(next.run(req).await)
}

/// Resolve by mw_req_stamp.
#[derive(Debug, Clone)]
pub struct ReqStamp {
    pub uuid: Uuid,
    pub time_in: OffsetDateTime,
}

// region:    --- ReqStamp Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for ReqStamp {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!("{:<12} - ReqStamp", "EXTRACTOR");
        parts
            .extensions
            .get::<ReqStamp>()
            .cloned()
            .ok_or(Error::ReqStampNotInResponseExt)
    }
}

// endregion: --- ReqStamp Extractor
