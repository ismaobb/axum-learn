use common::api_error::ApiError;
use entity::order::Entity as Order;
use sea_orm::{DbConn, EntityTrait};

use self::dto::WebOrderSource;

pub mod dto;

pub struct OrderService;

impl OrderService {
	pub async fn create_web_order_sources(
		conn: &DbConn,
		value: Vec<(entity::order::Model, Vec<entity::order_accessory::Model>)>,
	) -> Result<Vec<WebOrderSource>, ApiError> {
		let mut web_order_sources: Vec<WebOrderSource> = vec![];
		for order in value.into_iter() {
			let web_order_source = WebOrderSource::new(conn, (order.0, Some(order.1))).await?;
			web_order_sources.push(web_order_source);
		}

		Ok(web_order_sources)
	}

	pub async fn find_one(conn: &DbConn, id: u32) -> Result<WebOrderSource, ApiError> {
		let order = Order::find_by_id(id)
			.one(conn)
			.await?
			.ok_or(ApiError::NotFound(id.to_string()))?;

		tracing::info!(?order);
		WebOrderSource::new(conn, (order, None)).await
	}
}
