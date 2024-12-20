use axum::Router;

use chat_core::{AgentType, Chat, ChatAgent, ChatType, Message, User, Workspace};
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    error::ErrorOutput, handlers::*, AppState, CreateAgent, CreateChat, CreateMessage, CreateUser,
    ListMessage, SigninUser, UpdateAgent,
};

pub(crate) trait OpenApiRouter {
    fn openapi(self) -> Self;
}

#[derive(OpenApi)]
#[openapi(
        paths(
            signin_handler,
            signup_handler,
            create_chat_handler,
            list_chat_handler,
            get_chat_handler,
            list_message_handler,

            list_agent_handler,
            create_agent_handler,
            update_agent_handler
        ),
        modifiers(&SecurityAddon),
        components(
            schemas(
                User, Message, Chat, ChatType, Workspace,
                CreateChat, CreateMessage, CreateUser, ErrorOutput, AuthOutput,
                ListMessage, SigninUser, AuthOutput,
                CreateAgent, UpdateAgent, ChatAgent, AgentType
            )
        ),
        tags(
            (name = "chat-server", description = "chat-server management API")
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
