use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <button class="btn">{ "Hello daisyUI" }</button>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

