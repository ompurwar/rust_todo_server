use bson::doc;

use futures_util::StreamExt;
use log::{debug, error};
use mongodb::{options::FindOptions, Collection, Database};
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
    pub async fn get(&self) -> Result<Vec<Todo>, String> {
        let filter = doc! {};
        let mut vec = vec![];

        let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
        let cursor = self.todo_collection.find(filter, find_options).await;
        match cursor {
            Ok(mut c) => {
                while let Some(result) = c.next().await {
                    match result {
                        Ok(todo) => {
                            vec.push(todo);
                        }
                        Err(err) => return Err(err.to_string()),
                    }
                }
            }
            Err(err) => return Err(err.to_string()),
        }

        // Iterate over the results of the cursor.

        Ok(vec)
    }
}
