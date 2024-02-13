use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    html! {
        <>
            <div class="modal-container">
                <div class="modal">
                    // is it even legal to use this guy as a mascot
                    // i should really try to get someone to make a mascot for this
                    //
                    // <img
                    //    class="hero-emoji"
                    //    src="https://fonts.gstatic.com/s/e/notoemoji/latest/1f995/512.gif"
                    //    alt="🦕"
                    // />

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
