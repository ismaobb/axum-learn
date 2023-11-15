//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.5

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "lab_tds")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub r#type: String,
	#[sea_orm(primary_key)]
	pub lang: u8,
	pub auto_rule: Json,
	pub manager_rule: Option<Json>,
	pub director_rule: Option<Json>,
	pub retain_rule: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}