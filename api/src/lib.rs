mod doc;
mod user;

use std::{env, net::SocketAddr, str::FromStr, time::Duration};

use axum::{
	error_handling::HandleErrorLayer,
	http::{Method, StatusCode, Uri},
	routing::get,
	BoxError, Extension, Json, Router, Server,
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
	let server_url = format!("{host}:{port}");

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
		.layer(
			tower::ServiceBuilder::new()
				// `timeout` will produce an error if the handler takes
				// too long so we must handle those
				.layer(HandleErrorLayer::new(handle_timeout_error))
				.timeout(Duration::from_secs(3)),
		)
		// .layer(axum::middleware::from_extractor::<middleware::extract::JsonExtract>())
		.layer(axum::middleware::map_response(middleware::interceptor::transform))
		.layer(axum::middleware::map_response(middleware::filter::http_exception))
		.layer(trace_layer)
		.layer(Extension(state))
		.merge(doc::route());

	let addr = SocketAddr::from_str(&server_url).unwrap();
	tracing::info!("->> LISTENING on {addr}\n");
	Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
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
