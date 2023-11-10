use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use thiserror::Error;

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

#[derive(Serialize)]
pub struct ApiErrorResponse {
	message: Option<String>,
	code: u16,
}

impl IntoResponse for ApiErrorResponse {
	fn into_response(self) -> axum::response::Response {
		(StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR), Json(self)).into_response()
	}
}

#[derive(Error, Debug, Clone)]
pub enum UserError {
	#[error("User {0} Not Found")]
	UserNotFound(String),
	#[error("User Exist")]
	UserExist,
}

impl IntoResponse for UserError {
	fn into_response(self) -> axum::response::Response {
		let mut res = (StatusCode::INTERNAL_SERVER_ERROR).into_response();
		res.extensions_mut().insert(self);
		res
	}
}
