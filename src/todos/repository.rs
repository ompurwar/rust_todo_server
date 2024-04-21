use log::{debug, error};
use mongodb::{Collection, Database};

// src/todos/repository.rs
use super::entity::Todo;

pub struct TodoRepository {
    todo_collection: Collection<Todo>,
}

impl TodoRepository {
    pub fn new(db: &Database) -> Self {
        TodoRepository {
            todo_collection: db.collection::<Todo>("todos"),
        }
    }

    pub async fn save(&self, todo: &Todo) -> Result<String, String> {
        // Implement database saving logic here

        match self.todo_collection.insert_one(todo, None).await {
            Ok(result) => {
                debug!("{:#?}", result);
                Ok(result.inserted_id.to_string())
            }
            Err(err) => {
                error!("{:#?}", err);
                Err("todo nahi bna :(".to_string())
            }
        }
    }
}
