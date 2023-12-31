use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
        <div>
            <a href="https://github.com/Chibikuri"><img class="logo" src="assets/github-mark.png" alt="github" width="40px" height="auto"/></a>
            <a href="https://www.linkedin.com/in/ryosuke-satoh-50492817a/"><img class="logo" src="assets/Li-In-Bug.png" alt="LinkedIn" width="40px" height="auto"/></a>
        </div>
        {"This page is powered by "}<a href="https://yew.rs/">{"yew"}</a><br/>
        </footer>
    }
}
