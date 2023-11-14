use axum::{
	response::{IntoResponse, Response},
	Json,
};
use serde_json::json;

pub async fn http_exception(res: Response) -> impl IntoResponse {
	let service_error = res.extensions().get::<common::api_error::ApiError>();

	if let Some(err) = service_error {
		tracing::error!("{}", err.to_owned().to_string());

		(res.status(), Json(json!({"message:":err.to_string().to_owned(),"code":-2}))).into_response()
	} else {
		res
	}
}
