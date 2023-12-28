use yew::prelude::*;
use yew_router::prelude::*;
mod router;
pub mod pages {
    pub mod footer;
    pub mod header;
    pub mod home;
    pub mod tech_notes_index;
}

pub mod components {
    pub mod list;
    pub mod summary;
}

use pages::footer::Footer;
use pages::header::Header;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <BrowserRouter>
            <Header/>
            <router::Navi/>
            <Footer/>
        </BrowserRouter>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
