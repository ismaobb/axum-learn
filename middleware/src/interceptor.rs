use axum::response::{IntoResponse, Response};

pub async fn transform(mut res: Response) -> impl IntoResponse {
	res.headers_mut().insert("Server", "Axum".parse().unwrap());

	res
}
