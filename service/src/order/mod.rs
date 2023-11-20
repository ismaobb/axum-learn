use common::api_error::ApiError;
use entity::order::{Entity as Order, Model};
use sea_orm::{DbConn, EntityTrait};

use self::dto::WebOrderSource;

pub mod dto;

pub struct OrderService;

impl OrderService {
	pub async fn create_web_order_sources(conn: &DbConn, orders: Vec<Model>) -> Result<Vec<WebOrderSource>, ApiError> {
		let mut web_order_sources: Vec<WebOrderSource> = vec![];
		for order in orders.into_iter() {
			let web_order_source = WebOrderSource::new(conn, order).await?;
			web_order_sources.push(web_order_source);
		}

		Ok(web_order_sources)
	}

	pub async fn find_one(conn: &DbConn, id: u32) -> Result<WebOrderSource, ApiError> {
		let order = Order::find_by_id(id).one(conn).await?;
		tracing::info!(?order);
		match order {
			Some(v) => Ok(WebOrderSource::new(conn, v).await?),
			None => Err(ApiError::NotFound(id.to_string())),
		}
	}
}
