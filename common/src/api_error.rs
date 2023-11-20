use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
	#[error("{0}")]
	DbError(String),
	#[error("`{0}` Not Found")]
	NotFound(String),
	#[error("`{0}` Exist")]
	Exist(String),
}

impl From<sea_orm::DbErr> for ApiError {
	fn from(value: sea_orm::DbErr) -> Self {
		Self::DbError(value.to_string())
	}
}

impl IntoResponse for ApiError {
	fn into_response(self) -> axum::response::Response {
		let code = match self {
			ApiError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
			_ => StatusCode::BAD_REQUEST,
		};
		let mut res = code.into_response();
		res.extensions_mut().insert(self);
		res
	}
}
