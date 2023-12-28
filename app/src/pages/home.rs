use yew::prelude::*;

use crate::components::list::*;
use crate::components::summary::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1>{"Who am I?"}</h1>
            <ul>
              <li>{"Name: Ryosuke Satoh"}</li>
            </ul>
            <List name="Educations">
                <Summary summary="Keio University, Kanagawa Japan — Master of Media and Governance (Cyber Informatics Major)" details="tester"/>
                <Summary summary="Keio University, Kanagawa Japan — Bachelor of Environment and Information Studies" details="tester"/>
            </List>
            <List name="Work Experiences">
              <Summary summary="IBM Research Tokyo, Tokyo Japan (current)" details="Full time software engineer for IBM Quantum"/>
              <Summary summary="Keio Univesity, Tokyo, Japan" details="Full time researcher"/>
              <Summary summary="Milldea, Tokyo, Japan" details="Internship software engineer"/>
              <Summary summary="IBM Research, New York United States" details="Internship software engineer for IBM Quantum"/>
              <Summary summary="Mitou target project, Gate-type Quantum Computer (Ministry of Economy, Trade, and Industry)" details="Tokyo Japan — Self-employed (Software Engineer)"/>
            </List>
        </div>

    }
}
