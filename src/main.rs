use yew::prelude::*;
use yew_router::prelude::*;

#[path = "home.rs"] mod home;
use home::Home;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/neovolt/")]
    Home
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> }
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
