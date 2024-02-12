use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    html! {
        <>
            <div class="container">
                <h1>{ "Neovolt" }</h1>
                <p>{ "The cutest Revolt client of the yet" }</p>
            </div>
        </>
    }
}
