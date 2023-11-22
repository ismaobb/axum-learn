use std::time::Instant;

use crate::{order::dto::WebOrderSource, order_accessory::dto::PartialOrderDetail};

use self::dto::OrderAccessoryQuery;
use common::api_error::ApiError;
use entity::{prelude::*, *};
use sea_orm::{ColumnTrait, Condition, DbBackend, DbConn, EntityTrait, QueryFilter, QuerySelect, QueryTrait};
use sea_query::{Expr, MySqlQueryBuilder, Query};

pub mod dto;

pub struct OrderAccessoryService;

impl OrderAccessoryService {
	pub async fn find_all(conn: &DbConn, query: OrderAccessoryQuery) -> Result<Vec<WebOrderSource>, ApiError> {
		Self::example_at_lib_sea_query().unwrap();
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
					.add(order_detail::Column::Line.is_in(line.to_owned()))
					.add(order_detail::Column::State.gte(0))
					.add(order_accessory::Column::State.is_in(state.to_owned()))
					.add(accessory_column::Column::Follow.eq(1)),
			);
		let sql = select.build(DbBackend::MySql).to_string();
		tracing::info!(?sql);
		let orders = select.all(conn).await?;
		let web_order_sources = super::order::OrderService::create_web_order_sources(conn, orders).await?;
		let latency = Instant::now().duration_since(start).as_millis();
		tracing::info!(latency);

		let order_details = OrderDetail::find()
			.select_only()
			.columns([
				order_detail::Column::OrderId,
				order_detail::Column::Type,
				order_detail::Column::Thickness,
			])
			.distinct()
			.join(
				sea_orm::JoinType::LeftJoin,
				OrderDetail::belongs_to(OrderAccessory)
					.from(order_detail::Column::OrderId)
					.to(order_accessory::Column::OrderId)
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
			)
			.into_model::<PartialOrderDetail>()
			.all(conn)
			.await?;

		let web_order_sources = web_order_sources
			.into_iter()
			.map(|mut w| {
				w.details = order_details
					.iter()
					.filter_map(|o| if o.order_id == w.id { Some(o.to_owned()) } else { None })
					.collect::<Vec<PartialOrderDetail>>();
				w
			})
			.collect::<Vec<WebOrderSource>>();

		Ok(web_order_sources)
	}

	fn example_at_lib_sea_query() -> Result<(), ApiError> {
		let mut select = Query::select()
			.left_join(
				OrderAccessory,
				Expr::col((OrderDetail, order_detail::Column::OrderId))
					.equals((OrderAccessory, order_accessory::Column::OrderId)),
			)
			.left_join(
				AccessoryColumn,
				Expr::col((OrderAccessory, order_accessory::Column::Accessory))
					.equals((AccessoryColumn, accessory_column::Column::Accessory)),
			)
			.and_where(Expr::col(order_detail::Column::Line).eq(1))
			.to_owned();

		let sql = select
			.from(OrderDetail)
			.columns([
				(OrderDetail, order_detail::Column::OrderId),
				(OrderDetail, order_detail::Column::Type),
				(OrderDetail, order_detail::Column::Thickness),
			])
			.distinct()
			.to_string(MySqlQueryBuilder::default());

		tracing::info!(?sql);

		Ok(())
	}
}
