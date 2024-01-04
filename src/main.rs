use axum::{
    body::Body,
    http::{Method, Request, StatusCode, Uri},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
};
use cook_book::{
    config::{get_config, tracing::init_tracing},
    ctx::Ctx,
    error::Error,
    log_request,
    middle_ware::{
        auth::{ctx_resolver, require_auth},
        req_stamp::{self, ReqStamp},
    },
    routes_login,
};

use serde_json::json;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;
use tracing::Level;

use cook_book::error::Result;

#[tracing::instrument(skip_all)]
async fn hello_ok(req_stamp: ReqStamp) -> impl IntoResponse {
    tracing::info!("{req_stamp:?}");
    _ok().await
}

#[tracing::instrument(skip_all, fields(user.id = ctx.user_id()))]
async fn auth_hello_ok(ctx: Ctx, req_stamp: ReqStamp) -> impl IntoResponse {
    tracing::info!("{ctx:?}");
    tracing::info!("{req_stamp:?}");

    tracing::error!(
        "something bad happened to user: {user:?}",
        user = ctx.user_id()
    );

    _ok().await
}

#[tracing::instrument(skip_all)]
async fn _ok() -> impl IntoResponse {
    tracing::info!(test_ok = "ok attribute", "hello get endpoint called");
    tracing::info!("telling user hello");

    (StatusCode::OK, Html("<h1>Hello, World!</h1>"))
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let config = get_config();

    init_tracing()?;

    let routes_api: Router = Router::new()
        .route("/aok", get(auth_hello_ok))
        .route_layer(middleware::from_fn(require_auth));

    // Create a regular axum app.
    let app = Router::new()
        .route("/ok", get(hello_ok))
        .merge(crate::routes_login::routes())
        .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn(ctx_resolver))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                if let Some(stamp) = request.extensions().get::<ReqStamp>() {
                    tracing::span!(
                        Level::DEBUG,
                        "request",
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                        req_uuid = %stamp.uuid.to_string(),
                    )
                } else {
                    tracing::span!(
                        Level::DEBUG,
                        "request",
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                    )
                }
            }),
        )
        .layer(middleware::from_fn(req_stamp::request_stamp))
        .layer(CookieManagerLayer::new());

    // Create a `TcpListener` using tokio.
    let listener = TcpListener::bind((config.SERVICE_IP, config.SERVICE_PORT))
        .await
        .unwrap();
    println!(
        "Listening on: {:?}",
        listener.local_addr().expect("valid addr")
    );
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn main_response_mapper(
    ctx: Option<Ctx>,
    req_stamp: ReqStamp,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    // -- Get the eventual response error.
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // -- If client error, build the new reponse.
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": req_stamp.uuid.to_string(),
                }
            });

            println!("    ->> client_error_body: {client_error_body}");

            // Build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });

    // Build and log the server log line.
    let client_error = client_status_error.unzip().1;
    // TODO: Need to hander if log_request fail (but should not fail request)
    let _ =
        log_request::log_req(req_method, uri, ctx, req_stamp, service_error, client_error).await;

    println!();
    error_response.unwrap_or(res)
}
