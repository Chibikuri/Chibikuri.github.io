use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::home::Home;
use crate::pages::tech_notes_index::TechNoteIndex;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/tech_blog")]
    TechNoteIndex,
    #[at("/tech_blog/:id")]
    TechNote { id: u32 },
    #[not_found]
    #[at("/404")]
    NotFound, // This should be only for debugging
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home/> },
        Route::TechNoteIndex => html! {<TechNoteIndex/>},
        Route::TechNote { id } => html! { <h1> {"tech blog"} </h1>},
        Route::NotFound => html! {<h1> {"not found"}</h1>},
    }
}

#[function_component(Navi)]
pub fn navigator() -> Html {
    html! {
        <Switch<Route> render={switch} />
    }
}
