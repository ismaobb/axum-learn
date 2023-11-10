use axum::{
	http::{Method, Request, Uri},
	middleware::Next,
	response::{IntoResponse, Response},
};

pub async fn transform(mut res: Response) -> impl IntoResponse {
	res.headers_mut().insert("Server", "Axum".parse().unwrap());

	res
}

pub async fn logging<B>(uri: Uri, method: Method, req: Request<B>, next: Next<B>) -> Response {
	tracing::info!("{:<12} {method} - {uri}", "logging");

	next.run(req).await
}
