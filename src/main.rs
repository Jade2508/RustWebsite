
mod components {
    pub mod header;
    pub mod routers {
        pub mod router; // Adjusted for nested module structure
    }
}
mod pages {
    pub mod todo_list;
}


use yew::prelude::*;
use yew_router::prelude::*;
use components::header::Header;
use components::routers::router::{switch, AppRoute};

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header />
            <Switch<AppRoute> render={Callback::from(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
