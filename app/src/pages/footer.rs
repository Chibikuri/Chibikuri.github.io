use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
        {"This page is powered by "}<a href="https://yew.rs/">{"yew"}</a>
        <ul>
        </ul>
        </footer>
    }
}
