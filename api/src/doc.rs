use axum::Router;
use service::order::dto::WebOrderSource;
use service::order_accessory::dto::PartialOrderDetail;
use service::user::dto::{CreateUserDto, PatchUserDto, UserResponse};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{order, order_accessory, user};

#[derive(OpenApi)]
#[openapi(
    info(title="ims_oa",version="0.1.0",contact(name="maobb",url="https://github.com/ismaobb/axum-learn")),
    paths(
        user::get::find_one,
        user::get::find_all,
        user::post::create,
        user::patch::patch_one,
        order::get::find_one,
        order_accessory::get::find_all,
    ),
    tags(
        (name = "user", description = "用户"),
        (name = "order", description = "订单"),
        (name = "order-accessory", description = "订单辅材"),
    ),
    components(
        schemas(entity::user::Model,UserResponse,CreateUserDto,PatchUserDto,entity::order_accessory::Model,WebOrderSource,PartialOrderDetail)
    )
)]
struct ApiDoc;

pub fn route() -> Router {
	let mut api = ApiDoc::openapi();

	// let name = env::var("CARGO_PKG_NAME").unwrap();
	// let version = env::var("CARGO_PKG_VERSION").unwrap();
	// tracing::info!(?version);
	// tracing::info!(?name);

	api.info.version = String::from("0.2.0");
	api.info.title = String::from("ims_oa");

	Router::new().merge(SwaggerUi::new("/doc").url("/api-docs/openapi.json", api))
}
