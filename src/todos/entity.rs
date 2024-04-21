use bson::{doc, DateTime as BsonDateTime};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use super::dto::CreateTodoDto;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ToDoState {
    Todo,
    InProgress,
    Done,
}

// The Todo struct with serialization/deserialization, including a datetime field
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Todo {
    pub title: String,
    pub state: ToDoState,
    pub created_at: Option<BsonDateTime>, // Now using bson DateTime without chrono serialization
}

impl From<CreateTodoDto> for Todo {
    fn from(value: CreateTodoDto) -> Self {
        let chrono_dt = Utc::now();
        let bson_dt = BsonDateTime::from_chrono(chrono_dt);

        Todo {
            title: value.title,
            state: value.state,
            created_at: Some(bson_dt),
        }
    }
}
