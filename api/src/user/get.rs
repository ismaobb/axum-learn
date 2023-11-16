use axum::extract::{Extension, Path};
use axum_extra::extract::Query;
use common::{api_error::ApiError, api_ok::ApiOk};
use service::user::dto::{UserQuery, UserResponse};

use crate::AppState;

#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = u32, Path,)
    ),
    responses(
        (status = 200, body = UserResponse)
    ),
    tag = "user",
)]
pub async fn find_one(
	Extension(state): Extension<AppState>,
	Path(id): Path<i32>,
) -> Result<ApiOk<UserResponse>, ApiError> {
	let user = service::user::UserService::find_one(&state.conn, id).await?;
	Ok(ApiOk(user))
}

#[utoipa::path(
    get,
    path = "/users",
    params(UserQuery),
    responses(
        (status = 200, body = [UserResponse])
    ),
    tag = "user",
)]
pub async fn find_all(
	Extension(state): Extension<AppState>,
	Query(query): Query<UserQuery>,
) -> Result<ApiOk<Vec<UserResponse>>, ApiError> {
	let users = service::user::UserService::find_all(&state.conn, query).await?;
	Ok(ApiOk(users))
}
