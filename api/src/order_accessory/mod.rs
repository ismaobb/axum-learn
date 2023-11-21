pub mod get;

use axum::{routing::get, Router};

pub fn route() -> Router {
	Router::new().route("/", get(get::find_all))
}
