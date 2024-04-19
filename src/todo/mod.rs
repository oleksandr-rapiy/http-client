use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    id: i32,
    title: String,

    #[serde(rename = "isCompleted")]
    is_completed: bool,
}

impl Todo {
    pub fn to_todo_dto(&self) -> TodoDto {
        TodoDto {
            title: self.title.clone(),
            is_completed: self.is_completed,
        }
    }

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
