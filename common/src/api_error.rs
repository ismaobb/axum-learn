use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
	#[error("{0}")]
	DbError(#[from] sea_orm::DbErr),
	#[error("`{0}` Not Found")]
	NotFound(String),
	#[error("`{0}` Exist")]
	Exist(String),
}

impl IntoResponse for ApiError {
	fn into_response(self) -> axum::response::Response {
		let code = match self {
			ApiError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
			_ => StatusCode::BAD_REQUEST,
		};
		code.into_response()
	}
}
