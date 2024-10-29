use common::api_error::ApiError;
use entity::order::Entity as Order;
use sea_orm::{DbConn, EntityTrait};

use self::dto::WebOrderSource;

pub mod dto;

pub struct OrderService;

impl OrderService {
	pub async fn find_one(conn: &DbConn, id: u32) -> Result<WebOrderSource, ApiError> {
		let order = Order::find_by_id(id)
			.one(conn)
			.await?
			.ok_or(ApiError::NotFound(id.to_string()))?
			.into();

		Ok(order)
	}
}
