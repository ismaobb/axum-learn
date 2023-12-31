//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.5

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "recipe_name")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: i32,
	pub name: String,
	pub r#type: i32,
	pub ingredient: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
