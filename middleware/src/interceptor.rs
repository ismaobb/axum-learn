use axum::{
	http::{header, HeaderValue},
	response::{IntoResponse, Response},
};

pub async fn transform(mut res: Response) -> impl IntoResponse {
	res.headers_mut().insert(header::SERVER, "Axum".parse().unwrap());
	res.headers_mut()
		.insert(header::CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());

	res
}
