use crate::components::markdown::Markdown;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TechNoteProps {
    pub contents: String,
}

#[function_component(TechNote)]
pub fn tech_note(props: &TechNoteProps) -> Html {
    html! {
        <Markdown data={props.contents.clone()}/>
    }
}
