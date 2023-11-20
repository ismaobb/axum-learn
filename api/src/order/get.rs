use axum::{extract::Path, Extension};
use common::{api_error::ApiError, api_ok::ApiOk};
use service::order::dto::WebOrderSource;

use crate::AppState;

#[utoipa::path(
    get,
    path = "/orders/{id}",
    params(
        ("id" = u32, Path,)
    ),
    responses(
        (status = 200, body = WebOrderSource)
    ),
    tag = "order",
)]
pub async fn find_one(
	Extension(state): Extension<AppState>,
	Path(id): Path<u32>,
) -> Result<ApiOk<WebOrderSource>, ApiError> {
	let order = service::order::OrderService::find_one(&state.conn, id).await?;
	Ok(ApiOk(order))
}
