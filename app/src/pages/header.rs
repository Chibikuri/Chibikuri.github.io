use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <nav>
                <ul>
                    <li><Link<Route> to={Route::Home}>{ "Home" }</Link<Route>></li>
                    <li><Link<Route> to={Route::TechBlogIndex}>{ "Tech Blog" }</Link<Route>></li>
                </ul>
            </nav>
      </header>
    }
}
