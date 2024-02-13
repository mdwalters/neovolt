use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    html! {
        <>
            <div class="modal-container">
                <div class="modal">
                    <h1>{ "Neovolt" }</h1>
                    <p>{ "The cutest Revolt client of the yet" }</p>
                    
                    <small class="bottom">
                        { "Join the Revolt server: " }
                        <a href="https://rvlt.gg/Qzmc4b5p">{ "rvlt.gg/Qzmc4b5p" }</a>
                    </small>
                </div>
            </div>
        </>
    }
}
