use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::routers::router::AppRoute;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <nav>
            <Link<AppRoute> to={AppRoute::Home}>{"Home"}</Link<AppRoute>>
            <Link<AppRoute> to={AppRoute::Todo}>{"Todo"}</Link<AppRoute>>
        </nav>
    }
}
