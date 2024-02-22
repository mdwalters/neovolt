use yew::prelude::*;
// use yew_router::prelude::*;

#[function_component]
pub fn Login() -> Html {
    html! {
        <>
            <div class="modal-container">
                <div class="modal">
                    // dino :3
                    <img
                       class="hero-emoji"
                       src="https://fonts.gstatic.com/s/e/notoemoji/latest/1f995/512.gif"
                       alt="🦕"
                    />

                    <h1>{ "Neovolt" }</h1>
                    <p>{ "The cutest Revolt client of the yet" }</p>

                    <br />

                    <input type="email" placeholder="Email..." />
                    <br /><br />
                    <input type="email" placeholder="Password..." />

                    

                    <small class="bottom">
                        { "Join the Revolt server: " }
                        <a href="https://rvlt.gg/Qzmc4b5p">{ "rvlt.gg/Qzmc4b5p" }</a>
                        { " • " }
                        <a href="https://github.com/mdwalters/neovolt">{ "github.com/mdwalters/neovolt" }</a>
                        { " • Currently in alpha stage" }
                    </small>
                </div>
            </div>
        </>
    }
}
