pub mod get;
pub mod post;

use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;

pub fn route() -> Router {
	Router::new()
		.route("/:id", get(get::find_one))
		.route("/", get(get::find_all).post(post::create))
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct User {
	#[schema(example = "maobb")]
	name: String,
	// #[schema(example = 18)]
	// age: u8,
	#[schema(example = true)]
	is_old: Option<bool>,
}