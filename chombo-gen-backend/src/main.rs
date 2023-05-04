use log::LevelFilter;
use rocket::{launch, routes};

use crate::logging::init_logging;

mod errors;
mod logging;
mod render_hand;

#[launch]
fn rocket() -> _ {
    init_logging(LevelFilter::Debug).expect("Could not initialize logging");

    rocket::build().mount("/", routes![render_hand::render_hand])
}
