use super::entity::ToDoState;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CreateTodoDto {
    pub title: String,
    pub state: ToDoState,
}
