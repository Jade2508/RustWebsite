use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Clone, Properties, PartialEq)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub completed: bool,
    pub subtasks: Vec<Todo>,
}

#[function_component(TodoList)]
pub fn todo_list() -> Html {
    let todos = use_state(Vec::new);
    let new_todo_title = use_state(String::new);

    let on_add_todo = {
        let todos = todos.clone();
        let new_todo_title = new_todo_title.clone();
        Callback::from(move |_| {
            if !new_todo_title.is_empty() {
                let mut new_todos = (*todos).clone();
                new_todos.push(Todo {
                    id: new_todos.len() + 1,
                    title: (*new_todo_title).clone(),
                    completed: false,
                    subtasks: Vec::new(),
                });
                todos.set(new_todos);
                new_todo_title.set("".to_string()); // Clear the input field after adding
            }
        })
    };

    let on_toggle_todo = {
        let todos = todos.clone(); // This clones the state handle, not the todos themselves
        Callback::from(move |id: usize| {
            let mut new_todos = (*todos).clone(); // Now cloning the actual todos list
            if let Some(todo) = new_todos.iter_mut().find(|todo| todo.id == id) {
                todo.completed = !todo.completed;
            }
            todos.set(new_todos);
        })
    };



    html! {
    <>
        <input type="text"
            value={(*new_todo_title).clone()}
            oninput={Callback::from(move |e: InputEvent| {
                let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                new_todo_title.set(input.value());
            })}
        />
        <button onclick={on_add_todo.clone()}>{"Add Todo"}</button>
        <ul>
            { for todos.clone().to_vec().into_iter().map(|todo| {
                let on_toggle_todo = on_toggle_todo.clone(); // Clone inside the closure
                let todo_id = todo.id; // Extract todo id
                html! {
                    <>
                        <li key={todo.id} class={if todo.completed { "completed" } else { "" }}>
                            <input type="checkbox"
                                checked={todo.completed}
                                onclick={on_toggle_todo.reform(move |_| todo_id)} // Use cloned todo_id here
                            />
                            { &todo.title }
                        </li>
                    </>
                }
            })}
        </ul>
    </>
}
}
