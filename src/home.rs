use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    html! {
        <>
            <div class="container">
                <h1>{ "Neovolt" }</h1>
                <p>{ "The cutest Revolt client of the yet" }</p>
                <br />
                <small>{ "Join the Revolt server: " } <b><a href="https://rvlt.gg/Qzmc4b5p">{ "https://rvlt.gg/Qzmc4b5p" }</a></b></small>
            </div>
        </>
    }
}
