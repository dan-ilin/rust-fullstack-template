use yew::prelude::*;

mod components;

use crate::components::example::ExampleComponent;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    html! {
        <ExampleComponent />
    }
}
