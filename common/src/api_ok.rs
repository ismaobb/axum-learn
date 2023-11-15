use axum::{response::IntoResponse, Json};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub struct ApiOk<T: Serialize>(pub T);

impl<T: Serialize> IntoResponse for ApiOk<T> {
	fn into_response(self) -> axum::response::Response {
		Json(json!({"content":self.0,"code":0})).into_response()
	}
}
