use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::home::Home;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/tech_blog")]
    TechBlogIndex,
    #[at("/tech_blog/:id")]
    TechBlog { id: u32 },
    #[not_found]
    #[at("/404")]
    NotFound, // This should be only for debugging
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home/> },
        Route::TechBlogIndex => html! {<TechBlogIndex/>},
        Route::TechBlog { id } => html! { <h1> {"tech blog"} </h1>},
        Route::NotFound => html! {<h1> {"not found"}</h1>},
    }
}

#[function_component(TechBlogIndex)]
pub fn tech_blog_index() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
          <h1>{"Test"}</h1>
          <button {onclick}>{"Home"}</button>
        </div>
    }
}

#[function_component(Navi)]
pub fn navigator() -> Html {
    html! {
        <Switch<Route> render={switch} />
    }
}
