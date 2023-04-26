use std::fmt::Display;

use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlSelectElement};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props<T: PartialEq> {
    #[prop_or_default]
    pub id: AttrValue,
    pub options: Vec<T>,
    pub on_set_value: Callback<T>,
}

#[function_component]
pub fn Select<T>(props: &Props<T>) -> Html
where
    T: PartialEq + Copy + Default + Display + 'static,
{
    let on_change = {
        let props = props.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());

            if let Some(input) = input {
                let selected_index = input.selected_index() as usize;
                let selected_value = props.options[selected_index];
                props.on_set_value.emit(selected_value);
            }
        })
    };

    html! {
        <select onchange={on_change} class="form-select" aria-label="Tile Set" id={props.id.clone()}>
            {
                props.options.iter().map(|option| {
                    html!{<option value={ option.to_string() } selected={ *option == T::default() }>{ option.to_string() }</option>}
                }).collect::<Html>()
            }
        </select>
    }
}
