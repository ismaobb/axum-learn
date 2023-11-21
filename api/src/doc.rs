use std::env;

use axum::Router;
use service::order::dto::WebOrderSource;
use service::user::dto::{CreateUserDto, PatchUserDto, UserResponse};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{order, user};

#[derive(OpenApi)]
#[openapi(
    info(title="ims_oa",version="0.1.0"),
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
    components(schemas(entity::user::Model,UserResponse,CreateUserDto,PatchUserDto,entity::order_accessory::Model,WebOrderSource)
    )
)]
struct ApiDoc;

pub fn route() -> Router {
	let mut api = ApiDoc::openapi();

	let name = env::var("CARGO_PKG_NAME").unwrap();
	let version = env::var("CARGO_PKG_VERSION").unwrap();
	tracing::info!(?version);
	tracing::info!(?name);

	api.info.version = version;
	api.info.title = name;

	Router::new().merge(SwaggerUi::new("/doc").url("/api-docs/openapi.json", api))
}
