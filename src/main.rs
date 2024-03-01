use yew::prelude::*;
use yew_router::prelude::*;

mod pages;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/neovolt/")]
    Main,

    #[at("/neovolt/login")]
    Login,
    #[at("/neovolt/app")]
    Channels
}

fn switch(route: Route) -> Html {
    match route {
        Route::Main => html! { <Redirect<Route> to={Route::Login}/> },

        Route::Login => html! { <pages::Login /> },
        Route::Channels => html! { <pages::Channels /> }
    }
}


#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
