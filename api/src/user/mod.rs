pub mod get;
pub mod patch;
pub mod post;

use axum::{routing::get, Router};

pub fn route() -> Router {
	Router::new()
		.route("/:id", get(get::find_one).patch(patch::patch_one))
		.route("/", get(get::find_all).post(post::create))
}
