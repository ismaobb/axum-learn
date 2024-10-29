use axum::{extract::Path, Extension};
use common::{api_error::ApiError, api_ok::ApiOk};
use service::order::{dto::WebOrderSource, OrderService};

use crate::AppState;

#[utoipa::path(
    get,
    path = "/orders/{id}",
    params(
        ("id" = u32, Path,description="order id")
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
	let order = OrderService::find_one(&state.conn, id).await?;
	Ok(ApiOk(order))
}