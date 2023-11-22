use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, IntoParams)]
pub struct OrderAccessoryQuery {
	pub line: Vec<u32>,
	pub state: Vec<u32>,
}
#[derive(FromQueryResult, Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct PartialOrderDetail {
	#[serde(skip_serializing)]
	pub order_id: u32,
	pub r#type: String,
	pub thickness: u32,
}
