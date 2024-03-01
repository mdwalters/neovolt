use yew::prelude::*;

#[function_component]
pub fn Channels() -> Html {
    html! {
        <>
            <div class="modal-container">
                <div class="modal">
                    <h1>{ "The real thing!" }</h1>
                    <p>{ "...except its not here... yet." }</p>
                </div>
            </div>
        </>
    }
}
