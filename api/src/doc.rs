use axum::Router;
use service::user::dto::{CreateUserDto, UserResponse};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::user;

#[derive(OpenApi)]
#[openapi(
    paths(
        user::get::find_one,
        user::get::find_all,
        user::post::create,
    ),
    tags(
        (name = "user", description = "user"),
        (name = "user1", description = "user")
    ),
    components(schemas(entity::user::Model,UserResponse,CreateUserDto))
)]
struct ApiDoc;

pub fn route() -> Router {
	Router::new().merge(SwaggerUi::new("/doc").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
