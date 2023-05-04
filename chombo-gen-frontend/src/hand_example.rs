use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub hand: AttrValue,
    pub on_clicked: Callback<String>,
}

#[function_component]
pub fn HandExample(props: &Props) -> Html {
    let Props { hand, on_clicked } = props;

    let on_click = {
        let hand = hand.clone();
        let on_clicked = on_clicked.clone();

        Callback::from(move |_: MouseEvent| {
            on_clicked.emit(hand.to_string());
        })
    };

    html! {
        <button onclick={on_click} type="button" class="btn btn-link p-0"><samp>{ hand.clone() }</samp></button>
    }
}
