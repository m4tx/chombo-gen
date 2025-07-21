use std::time::Duration;

use async_trait::async_trait;
use cot::cli::CliMetadata;
use cot::config::{ProjectConfig, StaticFilesConfig, StaticFilesPathRewriteMode};
use cot::error::handler::DynErrorPageHandler;
use cot::openapi::swagger_ui::SwaggerUi;
use cot::project::{MiddlewareContext, RegisterAppsContext, RootHandler, RootHandlerBuilder};
use cot::router::method::openapi::api_get;
use cot::router::{Route, Router};
use cot::static_files::StaticFilesMiddleware;
use cot::{App, AppBuilder, Project, ProjectContext};
use tracing_subscriber::util::SubscriberInitExt;

use crate::errors::error_handler;
use crate::render_hand::new_render_hand;

mod errors;
mod render_hand;

struct ChomboGenApp;

#[async_trait]
impl App for ChomboGenApp {
    fn name(&self) -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    async fn init(&self, _context: &mut ProjectContext) -> cot::Result<()> {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .finish()
            .init();
        Ok(())
    }

    fn router(&self) -> Router {
        Router::with_urls([Route::with_api_handler("/hand/", api_get(new_render_hand))])
    }
}

struct ChomboGenProject;
impl Project for ChomboGenProject {
    fn cli_metadata(&self) -> CliMetadata {
        cot::cli::metadata!()
    }

    fn config(&self, _config_name: &str) -> cot::Result<ProjectConfig> {
        Ok(ProjectConfig::builder()
            .static_files(
                StaticFilesConfig::builder()
                    .rewrite(StaticFilesPathRewriteMode::QueryParam)
                    .cache_timeout(Duration::from_secs(365 * 24 * 60 * 60)) // 1 year
                    .build(),
            )
            .build())
    }

    fn register_apps(&self, apps: &mut AppBuilder, _context: &RegisterAppsContext) {
        apps.register_with_views(SwaggerUi::new(), "/swagger");
        apps.register_with_views(ChomboGenApp, "/api");
    }

    fn middlewares(&self, handler: RootHandlerBuilder, context: &MiddlewareContext) -> RootHandler {
        handler
            .middleware(StaticFilesMiddleware::from_context(context)) // needed for Swagger UI
            .build()
    }

    fn error_handler(&self) -> DynErrorPageHandler {
        DynErrorPageHandler::new(error_handler)
    }
}

#[cot::main]
fn main() -> impl Project {
    ChomboGenProject
}
