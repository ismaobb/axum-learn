//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.5

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "craft")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: u32,
	pub speed: Option<u32>,
	pub electrode: Option<String>,
	pub recipe_name: Option<String>,
	pub state: i8,
	pub origin: u8,
	pub production_serial: Option<i32>,
	pub thickness: i32,
	pub label_thickness: Option<i32>,
	pub length: Option<i32>,
	pub label_length: Option<i32>,
	pub width: Option<i32>,
	pub quality: Option<String>,
	pub level: Option<String>,
	pub is_revert: Option<String>,
	pub roll: Option<u32>,
	pub line: u8,
	pub r#type: String,
	pub produced_roll: u32,
	pub film_roll: u32,
	pub job_order: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
