use axum::{extract::Extension, Json};
use common::{api_error::ApiError, api_ok::ApiOk};
use entity::user::Model;

use crate::AppState;

#[utoipa::path(
    post,
    path = "/users",
    request_body = Model,
    responses(
        (status = 200, body = Model)
    ),
    tag = "user",
)]
pub async fn create(Extension(state): Extension<AppState>, Json(create_user): Json<Model>) -> Result<ApiOk<Model>, ApiError> {
	let user = service::user::UserService::create(&state.conn, create_user).await?;
	Ok(ApiOk(user))
}
