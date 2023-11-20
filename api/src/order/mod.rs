pub mod get;

use axum::{routing::get, Router};

pub fn route() -> Router {
	Router::new().route("/:id", get(get::find_one))
}
