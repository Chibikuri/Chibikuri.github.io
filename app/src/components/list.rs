use yew::prelude::*;

use super::summary::Summary;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub name: String,
    pub children: Html,
}

#[function_component(List)]
pub fn list(props: &Props) -> Html {
    html! {
        <div>
            <p> {props.name.clone()} </p>
            <ul>
                {props.children.clone()}
            </ul>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct ItemProp {
    pub value: String,
}

#[function_component(ListItem)]
pub fn list_item(item_prop: &ItemProp) -> Html {
    html! {
        <li>{format!("{}", item_prop.value.clone())}</li>
    }
}
