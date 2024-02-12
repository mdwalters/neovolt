use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <h1>{ "Neovolt" }</h1>
            <p>{ "The cutest Revolt client of the yet" }</p>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
