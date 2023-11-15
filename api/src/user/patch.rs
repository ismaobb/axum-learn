use axum::{extract::Path, Extension, Json};
use common::{api_error::ApiError, api_ok::ApiOk};
use entity::user::Model;
use service::user::dto::PatchUserDto;

use crate::AppState;

#[utoipa::path(
    patch,
    path = "/users/{id}",
    params(
        ("id" = u32, Path,)
    ),
    request_body = PatchUserDto,
    responses(
        (status = 200, body = Model)
    ),
    tag = "user",
)]
pub async fn patch_one(
	Extension(state): Extension<AppState>,
	Path(id): Path<i32>,
	Json(payload): Json<PatchUserDto>,
) -> Result<ApiOk<Model>, ApiError> {
	let user = service::user::UserService::patch_one(&state.conn, id, payload).await?;
	Ok(ApiOk(user))
}
