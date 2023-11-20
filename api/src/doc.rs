use axum::Router;
use entity::order_accessory;
use service::order::dto::WebOrderSource;
use service::user::dto::{CreateUserDto, PatchUserDto, UserResponse};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{order, user};

#[derive(OpenApi)]
#[openapi(
    paths(
        user::get::find_one,
        user::get::find_all,
        user::post::create,
        user::patch::patch_one,
        order::get::find_one,
    ),
    tags(
        (name = "user", description = "用户"),
        (name = "order", description = "订单")
    ),
    components(schemas(entity::user::Model,UserResponse,CreateUserDto,PatchUserDto,WebOrderSource,order_accessory::Model))
)]
struct ApiDoc;

pub fn route() -> Router {
	Router::new().merge(SwaggerUi::new("/doc").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
