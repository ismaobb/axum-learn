use std::time::Duration;

use axum::extract::{Extension, Json, Path, Query};
use common::api_response::{ApiOkResponse, UserError};
use serde::Deserialize;
use tokio::time::sleep;
use utoipa::IntoParams;

use crate::AppState;

use super::User;

#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = u32, Path, )
    ),
    responses(
        (status = 200, body = User)
    ),
    tag = "user",
)]
pub async fn find_one(Extension(state): Extension<AppState>, Path(id): Path<u32>) -> Result<Json<ApiOkResponse<User>>, UserError> {
	tracing::info!(state.pool);
	sleep(Duration::from_secs(1)).await;
	let user = User {
		name: id.to_string(),
		is_old: None,
	};

	Err(UserError::UserNotFound(user.name))
}

#[derive(Deserialize, IntoParams)]
pub struct UserQuery {
	name: Option<String>,
}

#[utoipa::path(
    get,
    path = "/users",
    params(UserQuery),
    responses(
        (status = 200, body = [User])
    ),
    tag = "user",
)]
pub async fn find_all(Extension(_state): Extension<AppState>, Query(query): Query<UserQuery>) -> Result<Json<ApiOkResponse<Vec<User>>>, UserError> {
	let mut users: Vec<User> = vec![];
	if query.name.is_some() {
		let name = query.name.unwrap();
		tracing::info!(name);

		users.push(User { name, is_old: None })
	}

	// Ok(Json(ApiSuccessResponse::new(users)))
	Err(UserError::UserExist)
}
