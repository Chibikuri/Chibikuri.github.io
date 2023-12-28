use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SummaryProps {
    pub summary: String,
    pub details: String,
}

#[function_component(Summary)]
pub fn summary(props: &SummaryProps) -> Html {
    html! {
        <details>
          <summary>{format!("{}", props.summary)}</summary>
          <p>{props.details.clone()}</p>
        </details>
    }
}
