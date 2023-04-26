use std::io::Cursor;
use std::time::Instant;

use chombo_gen_common::tile_set::TileSet;
use image::DynamicImage;
use image::ImageError;
use log::{error, info};
use riichi_hand::parser::{HandParseError, HandParser};
use riichi_hand::raster_renderer::fluffy_stuff_tile_sets::{
    BLACK_FLUFFY_STUFF_TILE_SET, RED_FLUFFY_STUFF_TILE_SET, YELLOW_FLUFFY_STUFF_TILE_SET,
};
use riichi_hand::raster_renderer::{RasterRenderer, RenderOptions};
use rocket::get;
use rocket::http::ContentType;
use rocket::tokio::task;
use rocket_cache_response::CacheResponse;
use time::Duration;

use crate::errors::ServiceError;

impl From<HandParseError> for ServiceError {
    fn from(error: HandParseError) -> Self {
        error!("{:?}", error);
        Self::BadRequest(error.to_string())
    }
}

impl From<ImageError> for ServiceError {
    fn from(error: ImageError) -> Self {
        error!("{:?}", error);
        Self::BadRequest(error.to_string())
    }
}

const CACHE_MAX_AGE: Duration = Duration::days(7);

#[get("/hand?<hand>&<tile_set>")]
pub async fn render_hand(
    hand: String,
    tile_set: TileSet,
) -> Result<(ContentType, CacheResponse<Vec<u8>>), ServiceError> {
    let tile_set = match tile_set {
        TileSet::Yellow => &*YELLOW_FLUFFY_STUFF_TILE_SET,
        TileSet::Red => &*RED_FLUFFY_STUFF_TILE_SET,
        TileSet::Black => &*BLACK_FLUFFY_STUFF_TILE_SET,
    };

    let hand_obj = HandParser::parse(&hand)?;

    let buf = task::spawn_blocking(move || {
        let render_time = Instant::now();
        let image = RasterRenderer::render(&hand_obj, tile_set, RenderOptions::default());
        let render_elapsed = render_time.elapsed();

        let create_png_time = Instant::now();
        let mut buf = Vec::new();
        DynamicImage::ImageRgba8(image)
            .write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png)?;
        let create_png_elapsed = create_png_time.elapsed();

        info!(
            "Hand {}: rendering took {:?}, PNG encoding took {:?}",
            hand, render_elapsed, create_png_elapsed
        );

        Ok::<Vec<u8>, ServiceError>(buf)
    })
    .await??;

    let cache = CacheResponse::Public {
        responder: buf,
        max_age: CACHE_MAX_AGE.whole_seconds() as u32,
        must_revalidate: false,
    };
    Ok((ContentType::PNG, cache))
}
