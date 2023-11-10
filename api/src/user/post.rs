use axum::{
	extract::Json,
	response::{IntoResponse, Response},
};
use common::api_response::ApiOkResponse;

use super::User;

#[utoipa::path(
    post,
    path = "/users",
    request_body = User,
    responses(
        (status = 200, body = User)
    ),
    tag = "user",
)]
pub async fn create(Json(user): Json<User>) -> Response {
	tracing::info!(user.name);
	ApiOkResponse::new(user).into_response()
}
