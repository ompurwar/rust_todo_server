use bson::{doc, DateTime as BsonDateTime};
use chrono::Utc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_bson_datetime"
    )]
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

fn serialize_bson_datetime<S>(date: &Option<BsonDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(d) => serializer.serialize_str(&d.to_chrono().to_rfc3339()),
        None => serializer.serialize_none(),
    }
}

fn deserialize_bson_datetime<'de, D>(deserializer: D) -> Result<Option<BsonDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    // Custom deserialization logic can be implemented if necessary
    unimplemented!()
}
