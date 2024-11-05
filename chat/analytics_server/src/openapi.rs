use axum::Router;

use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::{handler::*, AppState, ErrorOutput};

pub(crate) trait OpenApiRouter {
    fn openapi(self) -> Self;
}

#[derive(OpenApi)]
#[openapi(
        paths(
            create_event_handler,
        ),
        modifiers(&SecurityAddon),
        components(
            schemas(
                ErrorOutput
            )
        ),
        tags(
            (name = "analytics-server", description = "analytics-server management API")
        )
    )]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "token",
                SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).build()),
            )
        }
    }
}

impl OpenApiRouter for Router<AppState> {
    fn openapi(self) -> Self {
        self.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
            // There is no need to create `RapiDoc::with_openapi` because the OpenApi is served
            // via SwaggerUi instead we only make rapidoc to point to the existing doc.
            .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
    }
}
