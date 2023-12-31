use crate::fetcher;
use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(TechNoteIndex)]
pub fn tech_note_index() -> Html {
    let articles = vec![(String::from("test"), String::from("test"), 0)];
    let outline_view = articles.iter().map(|(article_title, outline, id)| {
        html! {
            <Card title={article_title.clone()} outline={outline.clone()} id={id.clone()}/>
        }
    });
    html! {
        <OutlineView>
        {for outline_view}
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
    pub id: u32,
}

#[function_component(Card)]
pub fn card_view(props: &CardProps) -> Html {
    html! {
        <Link<Route> to={Route::TechNote{id: props.id}}>
            <div class="card">
                <h5><b>{props.title.clone()}</b></h5>
            </div>
        </Link<Route>>
    }
}
