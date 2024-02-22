use yew::prelude::*;
use yew_router::prelude::*;

#[path = "login.rs"] mod login;
use login::Login;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/neovolt/")]
    Main,

    #[at("/neovolt/login")]
    Login
}

fn switch(route: Route) -> Html {
    match route {
        Route::Main => html! { <Redirect<Route> to={Route::Login}/> },
        Route::Login => html! { <Login /> }
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
