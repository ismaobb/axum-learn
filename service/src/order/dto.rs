use entity::order_accessory::Model;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::order_accessory::dto::PartialOrderDetail;

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct WebOrderSource {
	pub id: u32,
	pub job_order: String,
	pub deal_date: Option<String>,
	pub trade_mode: i8,
	pub delivery_time: Option<String>,
	pub density: f64,
	pub wrapper: Option<i32>,
	pub memo: Option<String>,
	pub produce_date: Option<String>,
	pub factory: u8,
	pub quality: Option<String>,
	pub level: Option<String>,
	pub scheme_date: Option<String>,
	pub wrapper_memo: Option<String>,
	pub plate_type: Option<String>,
	pub plate_size: Option<String>,
	pub plate_wrapper: Option<String>,
	pub roll_placement: Option<String>,
	pub other: Option<String>,
	pub packing: Option<String>,

	pub customer: Option<String>,
	pub salesman: String,
	#[schema(value_type = Vec<Model>)]
	pub accessories: Vec<Model>,

	pub details: Vec<PartialOrderDetail>,
}

impl From<(entity::order::Model, Vec<entity::order_accessory::Model>)> for WebOrderSource {
	fn from(value: (entity::order::Model, Vec<entity::order_accessory::Model>)) -> Self {
		let mut w = WebOrderSource::from(value.0);
		w.accessories = value.1;
		w
	}
}

impl From<entity::order::Model> for WebOrderSource {
	fn from(val: entity::order::Model) -> Self {
		let entity::order::Model {
			id,
			job_order,
			deal_date,
			trade_mode,
			delivery_time,
			density,
			wrapper,
			memo,
			produce_date,
			factory,
			quality,
			level,
			scheme_date,
			wrapper_memo,
			plate_type,
			plate_size,
			plate_wrapper,
			roll_placement,
			other,
			packing,
			..
		} = val;
		Self {
			id,
			job_order,
			deal_date: deal_date.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
			trade_mode,
			delivery_time: delivery_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
			density: format!("{:.3}", density).parse::<f64>().unwrap(),
			wrapper,
			memo,
			produce_date: produce_date.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
			factory,
			quality,
			level,
			scheme_date: scheme_date.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
			wrapper_memo,
			plate_type,
			plate_size,
			plate_wrapper,
			roll_placement,
			other,
			packing,
			customer: Default::default(),
			salesman: Default::default(),
			accessories: Default::default(),
			details: vec![],
		}
	}
}
