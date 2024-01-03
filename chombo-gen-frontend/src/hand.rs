use base64::Engine;
use chombo_gen_common::enums::EnumName;
use chombo_gen_common::tile_set::TileSet;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub hand: AttrValue,
    pub tile_set: TileSet,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceErrorResponse {
    message: String,
}

#[derive(Clone, Debug)]
pub enum HandState {
    Image(Vec<u8>),
    Error(ServiceErrorResponse),
    Loading,
    Empty,
}

impl Default for HandState {
    fn default() -> Self {
        Self::Empty
    }
}

const API_URL: Option<&str> = option_env!("CHOMBO_GEN_API_URL");

fn api_url() -> &'static str {
    API_URL.unwrap_or("/api")
}

#[function_component]
pub fn Hand(props: &Props) -> Html {
    let Props { hand, tile_set } = props;

    let hand_state = use_state(HandState::default);
    {
        let hand_state = hand_state.clone();
        let hand_val = hand.clone();
        let tile_set = *tile_set;

        use_effect_with((hand.clone(), tile_set), move |_| {
            if !hand_val.is_empty() {
                hand_state.set(HandState::Loading);
                let hand_state = hand_state.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let result = Request::get(&format!("{}/hand/", api_url()))
                        .query([("hand", hand_val)])
                        .query([("tile_set", tile_set.name())])
                        .send()
                        .await
                        .unwrap();

                    if result.ok() {
                        let image = result.binary().await.unwrap();
                        hand_state.set(HandState::Image(image));
                    } else {
                        let error: ServiceErrorResponse = result.json().await.unwrap();
                        hand_state.set(HandState::Error(error));
                    }
                });
            } else {
                hand_state.set(HandState::Empty);
            }
            || ()
        });
    }

    match (*hand_state).clone() {
        HandState::Image(img) => {
            let encoded_url = base64::engine::general_purpose::STANDARD.encode(img);
            let encoded_url = format!("data:image/png;base64,{}", encoded_url);
            html! {
                <>
                    <img src={ encoded_url.clone() } class="img-fluid mx-auto d-block mb-4" />
                    <div class="text-center">
                        <a class="btn btn-primary btn-lg text-center" href={ encoded_url } download="hand.png">
                            <i class="bi bi-download me-1"></i>
                            { "Save the image" }
                        </a>
                    </div>
                </>
            }
        }
        HandState::Error(error) => html! {
            <div class="alert alert-danger" role="alert">
                <strong>{ "Error: " }</strong>{ &error.message }
            </div>
        },
        HandState::Loading => html! {
            <div class="text-center">
                <div class="spinner-border" role="status">
                    <span class="visually-hidden">{ "Loading..." }</span>
                </div>
            </div>
        },
        HandState::Empty => html! {},
    }
}
