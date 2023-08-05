use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <p>{ "Hello World!" }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
