pub mod dto;
pub mod code_enum;

use common::api_error::ApiError;
use entity::user::{self, Entity as User};
use sea_orm::{
	ActiveModelTrait,
	ActiveValue::{NotSet, Set},
	ColumnTrait, Condition, DbConn, EntityTrait, Iterable, PaginatorTrait, QueryFilter, QuerySelect,
};

use self::dto::{PatchUserDto, UserQuery, UserResponse};

pub struct UserService;

impl UserService {
	pub async fn find_one(conn: &DbConn, id: i32) -> Result<UserResponse, ApiError> {
		User::find_by_id(id)
			.select_only()
			.columns(user::Column::iter().filter(|col| !matches!(col, user::Column::Password | user::Column::Sign)))
			.into_model()
			.one(conn)
			.await?
			.ok_or(ApiError::NotFound(id.to_string()))
	}

	pub async fn find_all(conn: &DbConn, query: UserQuery) -> Result<Vec<UserResponse>, ApiError> {
		// let a = query.role_type_not.as_ref().unwrap();
		// tracing::info!("{a:?}");
		User::find()
			.filter(
				Condition::all()
					.add_option(query.name.map(|name| user::Column::Name.eq(name)))
					.add_option(query.user.map(|user| user::Column::User.eq(user)))
					.add_option(query.role_type.map(|role_type| user::Column::RoleType.eq(role_type)))
					.add_option(
						query
							.role_type_not
							.map(|role_type_not| user::Column::RoleType.is_not_in(role_type_not)),
					),
			)
			.columns(user::Column::iter().filter(|col| !matches!(col, user::Column::Password | user::Column::Sign)))
			.into_model::<UserResponse>()
			.all(conn)
			.await
			.map_err(ApiError::DbError)
	}

	pub async fn create(conn: &DbConn, create_user: user::Model) -> Result<user::Model, ApiError> {
		// let u1 = create_user.to_owned().into_active_model();
		// u1.save(conn).await?.try_into_model()?;
		let exist = User::find()
			.filter(user::Column::User.eq(create_user.user.clone()))
			.count(conn)
			.await
			.is_ok_and(|count| count > 0);

		if exist {
			return Err(ApiError::Exist(create_user.user));
		}

		let u = user::ActiveModel {
			id: NotSet,
			name: Set(create_user.name),
			line: Set(create_user.line),
			role_type: Set(create_user.role_type),
			user: Set(create_user.user),
			password: Set(create_user.password),
			dep: Set(create_user.dep),
			sign: Set("sign".to_string()),
		};

		u.insert(conn).await.map_err(ApiError::DbError)
	}

	pub async fn patch_one(conn: &DbConn, id: i32, payload: PatchUserDto) -> Result<user::Model, ApiError> {
		let mut user: user::ActiveModel = User::find_by_id(id)
			.one(conn)
			.await?
			.ok_or(ApiError::NotFound(id.to_string()))
			.map(Into::into)?;

		if payload.role_type.is_some() {
			user.role_type = Set(payload.role_type);
		}

		if let Some(name) = payload.name {
			user.name = Set(name);
		}

		if let Some(password) = payload.password {
			user.password = Set(password);
		}

		tracing::info!("{user:?}");

		user.update(conn).await.map_err(ApiError::DbError)
	}
}
