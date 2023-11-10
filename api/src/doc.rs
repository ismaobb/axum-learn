use axum::Router;
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
    components(schemas(user::User))
)]
struct ApiDoc;

pub fn route() -> Router {
	Router::new().merge(SwaggerUi::new("/doc").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
