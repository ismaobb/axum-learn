use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams, Debug)]
pub struct UserQuery {
	pub user: Option<String>,
	pub name: Option<String>,
	pub role_type: Option<i32>,
	pub role_type_not: Option<Vec<i32>>,
}

#[derive(FromQueryResult, Debug, Serialize, ToSchema, Clone)]
pub struct UserResponse {
	pub id: i32,
	pub role_type: Option<i32>,
	pub name: String,
	pub user: String,
	pub dep: String,
	pub line: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct CreateUserDto {
	pub role_type: i32,
	pub name: String,
	pub user: String,
	pub dep: String,
	pub line: Value,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct PatchUserDto {
	pub role_type: Option<i32>,
	pub name: Option<String>,
	pub password: Option<String>,
}
