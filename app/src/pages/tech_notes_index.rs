use yew::prelude::*;
use std::path::PathBuf;
use crate::markdown_parser::parse_markdown;

// fn get_test() -> Html{
//     let path = PathBuf::from("notes/test.md");
//     let contents = parse_markdown(path);
//     let inner = Html::from_html_unchecked(AttrValue::from(contents));
//     inner
// }

#[function_component(TechNoteIndex)]
pub fn tech_note_index() -> Html {
    html! {
        <OutlineView>
          <Card title="test" outline="test"/>
          <Card title="test" outline="test"/>
        </OutlineView>
    }
}

#[derive(Properties, PartialEq)]
pub struct OutlineViewProps {
    pub children: Html,
}

#[function_component(OutlineView)]
pub fn outline_view(outline_view_props: &OutlineViewProps) -> Html {
    html! {
        {outline_view_props.children.clone()}
    }
}

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub title: String,
    pub outline: String,
}

#[function_component(Card)]
pub fn card_view(props: &CardProps) -> Html {
    html! {
        <div class="card">
          <h5><b>{props.title.clone()}</b></h5>
        </div>
    }
}
