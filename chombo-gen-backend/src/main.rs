use async_trait::async_trait;
use cot::cli::CliMetadata;
use cot::config::ProjectConfig;
use cot::openapi::swagger_ui::SwaggerUi;
use cot::project::{MiddlewareContext, RegisterAppsContext, RootHandlerBuilder};
use cot::router::method::openapi::api_get;
use cot::router::{Route, Router};
use cot::static_files::StaticFilesMiddleware;
use cot::{App, AppBuilder, BoxedHandler, Project, ProjectContext};
use log::LevelFilter;

use crate::logging::init_logging;
use crate::render_hand::new_render_hand;

mod errors;
mod logging;
mod render_hand;


struct ChomboGenApp;

#[async_trait]
impl App for ChomboGenApp {
    fn name(&self) -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    async fn init(&self, _context: &mut ProjectContext) -> cot::Result<()> {
        init_logging(LevelFilter::Debug).expect("Could not initialize logging");
        Ok(())
    }

    fn router(&self) -> Router {
        Router::with_urls([Route::with_api_handler("/hand", api_get(new_render_hand))])
    }
}

struct ChomboGenProject;
impl Project for ChomboGenProject {
    fn cli_metadata(&self) -> CliMetadata {
        cot::cli::metadata!()
    }

    fn config(&self, _config_name: &str) -> cot::Result<ProjectConfig> {
        Ok(ProjectConfig::default())
    }

    fn register_apps(&self, apps: &mut AppBuilder, _context: &RegisterAppsContext) {
        apps.register_with_views(SwaggerUi::new(), "/swagger");
        apps.register_with_views(ChomboGenApp, "");
    }

    fn middlewares(
        &self,
        handler: RootHandlerBuilder,
        context: &MiddlewareContext,
    ) -> BoxedHandler {
        handler
            .middleware(StaticFilesMiddleware::from_context(context))
            .build()
    }
}

#[cot::main]
fn main() -> impl Project {
    ChomboGenProject
}
