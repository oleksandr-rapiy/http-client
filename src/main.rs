use http_client::{
    client::{Client, HttpClient},
    todo::{Todo, TodoDto},
};
use reqwest::header::ACCEPT;

#[tokio::main]
async fn main() {
    const TODO_URL: &str = "http://localhost:5185/api/todo";
    let mut http_client = HttpClient::new(TODO_URL.to_string());
    let http_client = http_client.set_header(ACCEPT, String::from("application/json"));

    let todos = http_client.get::<Vec<Todo>>("").await.unwrap();

    print_todos(&todos);

    println!("Marking the first todo as completed");
    let todo = todos.last().unwrap();
    let todo_dto = todo.to_todo_dto().mark_as_completed();

    http_client
        .put::<TodoDto, Todo>(format!("/{}", todo.get_id()).as_str(), todo_dto)
        .await
        .unwrap();

    let todos = http_client.get::<Vec<Todo>>("").await.unwrap();

    print_todos(&todos);

    for todo in todos.iter() {
        http_client
            .delete::<()>(format!("/{}", todo.get_id()).as_str())
            .await;
    }
}

fn print_todos(todos: &Vec<Todo>) {
    for todo in todos.iter() {
        println!("{:?}", todo);
    }
}
