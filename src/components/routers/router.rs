use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::todo_list::TodoList;


#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/todo")]
    Todo,
}

pub fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <h1>{ "Home Page" }</h1> },
        AppRoute::Todo => html! { <TodoList /> },
    }
}
