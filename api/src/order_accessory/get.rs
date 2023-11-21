use axum::Extension;
use axum_extra::extract::Query;
use common::{api_error::ApiError, api_ok::ApiOk};
use service::{order::dto::WebOrderSource, order_accessory::dto::OrderAccessoryQuery};

use crate::AppState;

#[utoipa::path(
    get,
    path = "/order-accessories",
    params(OrderAccessoryQuery),
    responses(
        (status = 200, body = [WebOrderSource])
    ),
    tag = "order-accessory",
)]
pub async fn find_all(
	Extension(state): Extension<AppState>,
	Query(query): Query<OrderAccessoryQuery>,
) -> Result<ApiOk<Vec<WebOrderSource>>, ApiError> {
	let orders = service::order_accessory::OrderAccessoryService::find_all(&state.conn, query).await?;
	Ok(ApiOk(orders))
}
