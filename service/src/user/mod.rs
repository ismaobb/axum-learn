pub mod dto;

use common::api_error::ApiError;
use entity::user::{self, Entity as User};
use sea_orm::{
	ActiveModelTrait, ActiveValue::NotSet, ActiveValue::Set, ColumnTrait, Condition, DbConn, EntityTrait, Iterable, QueryFilter, QuerySelect,
	TryIntoModel,
};

use self::dto::{CreateUserDto, PatchUserDto, UserQuery, UserResponse};

pub struct UserService;

impl UserService {
	pub async fn find_one(conn: &DbConn, id: i32) -> Result<UserResponse, ApiError> {
		User::find_by_id(id)
			.select_only()
			.columns(user::Column::iter().filter(|col| !matches!(col, user::Column::Password | user::Column::Sign)))
			.into_model()
			.one(conn)
			.await?
			.ok_or(ApiError::UserNotFound(id.to_string()))
	}

	pub async fn find_all(conn: &DbConn, query: UserQuery) -> Result<Vec<UserResponse>, ApiError> {
		User::find()
			.filter(
				Condition::all()
					.add_option(query.name.map(|name| user::Column::Name.eq(name)))
					.add_option(query.user.map(|user| user::Column::User.eq(user)))
					.add_option(query.role_type.map(|role_type| user::Column::RoleType.eq(role_type)))
					.add_option(query.role_type_not.map(|role_type_not| user::Column::RoleType.is_not_in(role_type_not))),
			)
			.columns(user::Column::iter().filter(|col| !matches!(col, user::Column::Password | user::Column::Sign)))
			.into_model::<UserResponse>()
			.all(conn)
			.await
			.map_err(|error| ApiError::DbError(error.to_string()))
	}

	pub async fn create(_conn: &DbConn, _create_user: CreateUserDto) -> Result<user::Model, ApiError> {
		let u = user::ActiveModel {
			id: NotSet,
			name: sea_orm::ActiveValue::Set("maobb".to_string()),
			..Default::default()
		};

		let u1 = u.try_into_model().map_err(|error| ApiError::DbError(error.to_string()))?;
		// let user = u.insert(conn).await.map_err(|error| ApiError::DbError(error.to_string()))?;
		Ok(u1)
	}

	pub async fn patch_one(conn: &DbConn, id: i32, payload: PatchUserDto) -> Result<user::Model, ApiError> {
		let mut user: user::ActiveModel = User::find_by_id(id)
			.one(conn)
			.await?
			.ok_or(ApiError::UserNotFound(id.to_string()))
			.map(Into::into)?;

		tracing::info!("{user:?}");
		user.role_type = Set(payload.role_type);
		if payload.name.is_some() {
			user.name = Set(payload.name.unwrap());
		}
		// user.password = Set(payload.password);
		// user.line = Set(payload.line);
		user.update(conn).await.map_err(|error| ApiError::DbError(error.to_string()))
	}
}