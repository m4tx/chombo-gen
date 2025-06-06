use std::io::Cursor;
use std::time::Instant;

use bytes::Bytes;
use chombo_gen_common::tile_set::TileSet;
use cot::aide::openapi::Operation;
use cot::openapi::{ApiOperationResponse, RouteContext};
use cot::request::extractors::UrlQuery;
use cot::response::{IntoResponse, Response};
use cot::{Body, StatusCode};
use image::{DynamicImage, ImageError};
use indexmap::IndexMap;
use riichi_hand::parser::{HandParseError, HandParser};
use riichi_hand::raster_renderer::fluffy_stuff_tile_sets::{
    BLACK_FLUFFY_STUFF_TILE_SET, RED_FLUFFY_STUFF_TILE_SET, YELLOW_FLUFFY_STUFF_TILE_SET,
};
use riichi_hand::raster_renderer::martin_persson_tile_sets::MARTIN_PERSSON_TILE_SET;
use riichi_hand::raster_renderer::{HandRenderError, RasterRenderer, RenderOptions};
use schemars::{JsonSchema, SchemaGenerator};
use serde::Deserialize;
use time::Duration;
use tracing::{error, info};

use crate::errors::ServiceError;

impl From<HandParseError> for ServiceError {
    fn from(error: HandParseError) -> Self {
        error!("{error:?}");
        Self::BadRequest(error.to_string())
    }
}

impl From<ImageError> for ServiceError {
    fn from(error: ImageError) -> Self {
        error!("{error:?}");
        Self::BadRequest(error.to_string())
    }
}

impl From<HandRenderError> for ServiceError {
    fn from(error: HandRenderError) -> Self {
        error!("{error:?}");
        Self::BadRequest(error.to_string())
    }
}

const MAX_HAND_LEN: usize = 100;
const CACHE_MAX_AGE: Duration = Duration::days(7);

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct RenderHandParams {
    hand: String,
    tile_set: TileSet,
}

pub async fn new_render_hand(
    UrlQuery(RenderHandParams { hand, tile_set }): UrlQuery<RenderHandParams>,
) -> Result<ImageResponse, ServiceError> {
    if hand.len() > MAX_HAND_LEN {
        return Err(ServiceError::BadRequest(format!(
            "Maximum hand description length exceeded ({}/{} characters)",
            hand.len(),
            MAX_HAND_LEN
        )));
    }

    let hand_obj = HandParser::parse(&hand)?;

    let buf = tokio::task::spawn_blocking(move || {
        let render_time = Instant::now();
        let options = RenderOptions::default();
        let image = match tile_set {
            TileSet::Yellow => {
                RasterRenderer::render(&hand_obj, &*YELLOW_FLUFFY_STUFF_TILE_SET, options)
            }
            TileSet::Red => RasterRenderer::render(&hand_obj, &*RED_FLUFFY_STUFF_TILE_SET, options),
            TileSet::Black => {
                RasterRenderer::render(&hand_obj, &*BLACK_FLUFFY_STUFF_TILE_SET, options)
            }
            TileSet::MartinPersson => {
                RasterRenderer::render(&hand_obj, &*MARTIN_PERSSON_TILE_SET, options)
            }
        }?;
        let render_elapsed = render_time.elapsed();

        let create_png_time = Instant::now();
        let mut buf = Vec::new();
        DynamicImage::ImageRgba8(image)
            .write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)?;
        let create_png_elapsed = create_png_time.elapsed();

        info!(
            "Hand {hand}: rendering took {render_elapsed:?}, PNG encoding took {create_png_elapsed:?}"
        );

        Ok::<Vec<u8>, ServiceError>(buf)
    })
    .await??;

    Ok(ImageResponse::new(buf))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImageResponse {
    image: Bytes,
}

impl ImageResponse {
    #[must_use]
    pub fn new(image: Vec<u8>) -> Self {
        Self {
            image: Bytes::from(image),
        }
    }
}

impl IntoResponse for ImageResponse {
    fn into_response(self) -> cot::Result<Response> {
        Response::new(Body::fixed(self.image))
            .with_header(cot::http::header::CONTENT_TYPE, "image/png")
            .with_header(
                cot::http::header::CACHE_CONTROL,
                format!("public, max-age={}", CACHE_MAX_AGE.whole_seconds()),
            )
            .into_response()
    }
}

impl ApiOperationResponse for ImageResponse {
    fn api_operation_responses(
        _operation: &mut Operation,
        _route_context: &RouteContext<'_>,
        _schema_generator: &mut SchemaGenerator,
    ) -> Vec<(
        Option<cot::aide::openapi::StatusCode>,
        cot::aide::openapi::Response,
    )> {
        vec![(
            Some(cot::aide::openapi::StatusCode::Code(
                StatusCode::OK.as_u16(),
            )),
            cot::aide::openapi::Response {
                description: "Image".to_string(),
                content: IndexMap::from([(
                    "image/png".to_string(),
                    cot::aide::openapi::MediaType {
                        schema: None,
                        ..Default::default()
                    },
                )]),
                ..Default::default()
            },
        )]
    }
}
