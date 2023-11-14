use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiOkResponse<T: Serialize> {
	content: T,
	code: u8,
}

impl<T: Serialize> IntoResponse for ApiOkResponse<T> {
	fn into_response(self) -> axum::response::Response {
		Json(self).into_response()
	}
}

impl<T: Serialize> ApiOkResponse<T> {
	pub fn new(content: T) -> Self {
		Self { content, code: 0 }
	}
}
