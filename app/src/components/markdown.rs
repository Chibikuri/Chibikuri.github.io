use pulldown_cmark::{html::push_html, Parser};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MdProps {
    pub data: String,
}

#[function_component(Markdown)]
pub fn markdown(props: &MdProps) -> Html {
    let parser = Parser::new(&props.data);
    let mut html = String::new();
    push_html(&mut html, parser);
    html! {
        {Html::from_html_unchecked(AttrValue::from(html))}
    }
}
