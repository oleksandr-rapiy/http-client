use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    id: i32,
    title: String,

    #[serde(rename = "isCompleted")]
    is_completed: bool,
}

impl Todo {
    pub fn get_id(&self) -> i32 {
        self.id
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoDto {
    title: String,

    #[serde(rename = "isCompleted")]
    is_completed: bool,
}

impl TodoDto {
    pub fn new(title: String, is_completed: bool) -> TodoDto {
        TodoDto {
            title,
            is_completed,
        }
    }

    pub fn mark_as_completed(&mut self) -> Self {
        self.is_completed = true;

        return self.to_owned();
    }
}

pub struct Mapper;

impl Mapper {
    pub fn to_todo_dto(todo: &Todo) -> TodoDto {
        TodoDto {
            title: todo.title.clone(),
            is_completed: todo.is_completed,
        }
    }
}