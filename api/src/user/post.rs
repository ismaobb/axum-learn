use axum::{extract::Extension, Json};
use common::{api_error::ApiError, api_response::ApiOkResponse};
use entity::user::Model;
use service::user::dto::CreateUserDto;

use crate::AppState;

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserDto,
    responses(
        (status = 200, body = Model)
    ),
    tag = "user",
)]
pub async fn create(Extension(state): Extension<AppState>, Json(create_user): Json<CreateUserDto>) -> Result<ApiOkResponse<Model>, ApiError> {
	tracing::info!(create_user.name);
	let user = service::user::UserService::create(&state.conn, create_user).await?;
	Ok(ApiOkResponse::new(user))
}
