use chombo_gen_common::tile_set::TileSet;
use yew::prelude::*;

use crate::hand::Hand;
use crate::hand_example::HandExample;
use crate::input::Input;
use crate::select::Select;

#[function_component]
pub fn HandGenerator() -> Html {
    let hand = use_state(String::default);
    let hand_rendered = use_state(String::default);
    let tile_set = use_state(TileSet::default);

    let on_tile_set_select: Callback<TileSet> = {
        let tile_set = tile_set.clone();

        Callback::from(move |value: TileSet| {
            tile_set.set(value);
        })
    };

    let on_hand_entry: Callback<String> = {
        let hand_rendered = hand_rendered.clone();

        Callback::from(move |value: String| {
            hand_rendered.set(value);
        })
    };

    let on_hand_debounce: Callback<String> = {
        let hand = hand.clone();
        let hand_rendered = hand_rendered.clone();

        Callback::from(move |value: String| {
            hand.set(value.clone());
            hand_rendered.set(value);
        })
    };

    let options = vec![TileSet::Yellow, TileSet::Red, TileSet::Black];

    html! {
        <>
            <h1>{ "Riichi Hand Generator" }</h1>

            <div class="mb-3">
                <label for="tile-set-select" class="form-label">{ "Tile Set" }</label>
                <Select<TileSet> on_set_value={ on_tile_set_select } { options } id="tile-set-select" />
            </div>
            <div class="mb-3">
                <label for="hand" class="form-label">{ "Hand" }</label>
                <Input on_set_value={ on_hand_entry.clone() } on_debounce={ on_hand_debounce.clone() } value={ (*hand_rendered).clone() } id="hand" />
            </div>

            <p>
                <button type="button" class="btn btn-outline-light btn-sm" data-bs-target="#collapseHelp" data-bs-toggle="collapse" aria-expanded="false" aria-controls="collapseHelp">{ "Hand format help" }</button>
            </p>

            <div class="collapse" id="collapseHelp">
                <div class="card card-body mb-3">
                    { "Hands need to be represented in a text format. The format is described below." }
                    <ul>
                        <li>
                            <strong>{ "Numerical tiles" }</strong> {" consist of a digit and a letter representing the tile suite (" }<samp>{ "m" }</samp>{ ", " }<samp>{ "s" }</samp>{ ", " }<samp>{ "p" }</samp>{ " for manzu, souzu, and pinzu, respectively)." }
                            <ul>
                                <li>{ "Zero means a " }<strong>{ "red five" }</strong>{ "." }</li>
                                <li>{ "Examples: " }<HandExample hand="1s" on_clicked={ on_hand_debounce.clone() } />{ " (1 of bamboos), " }<HandExample hand="5p" on_clicked={ on_hand_debounce.clone() } />{ " (5 of circles), " }<HandExample hand="3m" on_clicked={ on_hand_debounce.clone() } />{ " (3 of characters), " }<HandExample hand="0s" on_clicked={ on_hand_debounce.clone() } />{ " (red 5 of bamboos)." }</li>
                            </ul>
                        </li>
                        <li>
                            <strong>{ "Honor tiles" }</strong>{ " are represented using the tile suite " }<samp>{ "z" }</samp>{ ". Tiles 1-4 are wind tiles (East, South, West, North), and 5-7 are dragon tiles (White, Green, Red)." }
                            <ul>
                                <li>{ "Examples: " }<HandExample hand="1z" on_clicked={ on_hand_debounce.clone() } />{ " (East), " }<HandExample hand="6z" on_clicked={ on_hand_debounce.clone() } />{ " (green dragon)." }</li>
                                <li>{ "For convenience, single-character format is also available. " }<HandExample hand="E" on_clicked={ on_hand_debounce.clone() } />{ ", " }<HandExample hand="S" on_clicked={ on_hand_debounce.clone() } />{ ", " }<HandExample hand="W" on_clicked={ on_hand_debounce.clone() } />{ ", " }<HandExample hand="N" on_clicked={ on_hand_debounce.clone() } />{ " is East, South, West, and North, respectively, while " }<HandExample hand="w" on_clicked={ on_hand_debounce.clone() } />{ ", " }<HandExample hand="g" on_clicked={ on_hand_debounce.clone() } />{ ", " }<HandExample hand="r" on_clicked={ on_hand_debounce.clone() } />{ " are white, green, and red dragon, respectively." }</li>
                            </ul>
                        </li>
                        <li>{ "To get a reversed tile, " }<HandExample hand="?" on_clicked={ on_hand_debounce.clone() } />{ " can be used. Example: " }<HandExample hand="?33m?" on_clicked={ on_hand_debounce.clone() } />{ " (closed kan of 3 of characters)." }</li>
                        <li>{ "In case of longer sequences of tiles in " }<strong>{ "the same suite" }</strong>{ ", the tile suite characters can be omitted except for the last one. Example: " }<HandExample hand="123s" on_clicked={ on_hand_debounce.clone() } />{ " is the same as " }<HandExample hand="1s2s3s" on_clicked={ on_hand_debounce.clone() } />{ "." }</li>
                        <li>{ "Tiles can be " }<strong>{ "rotated" }</strong>{ " by putting an asterisk (" }<samp>{ "*" }</samp>{ ") after the tile, or " }<strong>{ "rotated and shifted" }</strong>{ " with double asterisk (" }<samp>{ "**" }</samp>{ "). Examples: " }<HandExample hand="1*s" on_clicked={ on_hand_debounce.clone() } />{ " (rotated 1 of bamboos), " }<HandExample hand="3*3**33p" on_clicked={ on_hand_debounce.clone() } />{ " (open kan of 3 of circles)." }</li>
                        <li>{ "Groups of tiles can be " }<strong>{ "separated" }</strong>{ " using an underscore (" }<samp>{ "_" }</samp>{ "). Example: " }<HandExample hand="123s_4*56s" on_clicked={ on_hand_debounce.clone() } />{ "." }</li>
                    </ul>

                    { "More examples:" }
                    <ul>
                        <li><HandExample hand="123s345m345m222pWW" on_clicked={ on_hand_debounce.clone() } /></li>
                        <li><HandExample hand="EE_w*ww_gg*g_rrr*_?WW?" on_clicked={ on_hand_debounce.clone() } /></li>
                        <li><HandExample hand="22m11s33s77pEEggrr" on_clicked={ on_hand_debounce.clone() } /></li>
                    </ul>
                </div>
            </div>

            <Hand hand={ (*hand).clone() } tile_set={ *tile_set } />
        </>
    }
}
