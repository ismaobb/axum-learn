mod doc;
mod order;
mod order_accessory;
mod user;

use std::{env, time::Duration};

use axum::{
	error_handling::HandleErrorLayer,
	http::{Method, StatusCode, Uri},
	routing::get,
	BoxError, Extension, Json, Router,
};
use sea_orm::DbConn;
use serde_json::json;

use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

#[tokio::main]
pub async fn bootstrap() {
	let version = env!("CARGO_PKG_VERSION");
	let host = env::var("HOST").expect("HOST is not set in .env file");
	let port = env::var("PORT").expect("PORT is not set in .env file");
	let addr = format!("{host}:{port}");

	let trace_layer = TraceLayer::new_for_http()
		.make_span_with(DefaultMakeSpan::new().level(Level::INFO))
		.on_request(DefaultOnRequest::new().level(Level::INFO))
		.on_response(DefaultOnResponse::new().level(Level::INFO));

	let state = AppState {
		conn: database::init_db().await,
	};

	// build our application with a single route
	let app = Router::new()
		.route("/", get(|| async { Json(json!({"version":version.to_owned()})) }))
		.nest("/users", user::route())
		.nest("/orders", order::route())
		.nest("/order-accessories", order_accessory::route())
		.layer(
			tower::ServiceBuilder::new()
				// `timeout` will produce an error if the handler takes
				// too long so we must handle those
				.layer(HandleErrorLayer::new(handle_timeout_error))
				.timeout(Duration::from_secs(3)),
		)
		.layer(axum::middleware::map_response(middleware::interceptor::transform))
		.layer(axum::middleware::map_response(middleware::filter::http_exception))
		.layer(trace_layer)
		.layer(Extension(state))
		.merge(doc::route());

	tracing::info!("->> LISTENING on {addr}\n");
	let tcp_listener = tokio::net::TcpListener::bind(addr).await.unwrap();
	axum::serve(tcp_listener, app.into_make_service()).await.unwrap();
}

#[derive(Clone)]
pub struct AppState {
	conn: DbConn,
}

async fn handle_timeout_error(
	// `Method` and `Uri` are extractors so they can be used here
	method: Method,
	uri: Uri,
	// the last argument must be the error itself
	err: BoxError,
) -> (StatusCode, String) {
	(
		StatusCode::INTERNAL_SERVER_ERROR,
		format!("`{} {}` failed with {}", method, uri, err),
	)
}
