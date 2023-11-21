use std::time::Instant;

use crate::order::dto::WebOrderSource;

use self::dto::OrderAccessoryQuery;
use common::api_error::ApiError;
use entity::{prelude::*, *};
use sea_orm::{ColumnTrait, Condition, DbBackend, DbConn, EntityTrait, QueryFilter, QuerySelect, QueryTrait};

pub mod dto;

pub struct OrderAccessoryService;

impl OrderAccessoryService {
	pub async fn find_all(conn: &DbConn, query: OrderAccessoryQuery) -> Result<Vec<WebOrderSource>, ApiError> {
		let start = Instant::now();
		let OrderAccessoryQuery { line, state } = query;
		let select = Order::find()
			.distinct()
			.join(
				sea_orm::JoinType::LeftJoin,
				Order::belongs_to(OrderAccessory)
					.from(order::Column::Id)
					.to(order_accessory::Column::OrderId)
					.into(),
			)
			.join(
				sea_orm::JoinType::LeftJoin,
				Order::belongs_to(OrderDetail)
					.from(order::Column::Id)
					.to(order_detail::Column::OrderId)
					.into(),
			)
			.join(
				sea_orm::JoinType::LeftJoin,
				OrderAccessory::belongs_to(AccessoryColumn)
					.from(order_accessory::Column::Accessory)
					.to(accessory_column::Column::Accessory)
					.into(),
			)
			.filter(
				Condition::all()
					.add(order_detail::Column::Line.is_in(line))
					.add(order_detail::Column::State.gte(0))
					.add(order_accessory::Column::State.is_in(state))
					.add(accessory_column::Column::Follow.eq(1)),
			);
		let sql = select.build(DbBackend::MySql).to_string();
		tracing::info!(?sql);
		let orders = select.all(conn).await?;
		let web_order_sources = super::order::OrderService::create_web_order_sources(conn, orders).await?;
		let latency = Instant::now().duration_since(start).as_millis();
		tracing::info!(latency);

		Ok(web_order_sources)
	}
}
