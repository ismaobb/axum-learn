use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct OrderAccessoryQuery {
	pub line: Vec<u32>,
	pub state: Vec<u32>,
}
