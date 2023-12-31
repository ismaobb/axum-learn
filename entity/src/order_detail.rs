//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.5

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "order_detail")]
pub struct Model {
	pub order_id: u32,
	pub index: u32,
	pub r#type: String,
	pub thickness: u32,
	#[sea_orm(column_type = "Double", nullable)]
	pub actual_thickness: Option<f64>,
	pub width: u32,
	pub length: u32,
	pub actual_length: Option<u32>,
	pub rolls: u32,
	#[sea_orm(column_type = "Float")]
	pub weight: f32,
	#[sea_orm(column_type = "Float", nullable)]
	pub produced_weight: Option<f32>,
	pub produced_rolls: Option<i32>,
	pub delivery_date: Option<DateTime>,
	pub corona: Option<String>,
	#[sea_orm(primary_key)]
	pub id: i32,
	pub line: Option<u8>,
	pub state: i8,
	pub origin: u8,
	pub scheme_rolls: u32,
	pub slice_rolls: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
