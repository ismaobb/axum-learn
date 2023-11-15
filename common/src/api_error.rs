use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
	#[error("{0}")]
	DbError(String),
	#[error("User {0} Not Found")]
	UserNotFound(String),
	#[error("User Exist")]
	UserExist,
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
