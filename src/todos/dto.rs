use super::entity::{ToDoState, Todo};
use bson::DateTime;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Deserialize, Clone)]
pub struct CreateTodoDto {
    pub title: String,
    pub state: ToDoState,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetTodoDto {
    pub title: String,
    pub state: ToDoState,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_bson_datetime"
    )]
    pub created_at: Option<DateTime>,
}

impl From<Todo> for GetTodoDto {
    fn from(value: Todo) -> Self {
        GetTodoDto {
            title: value.title,
            state: value.state,
            created_at: value.created_at,
        }
    }
}

fn serialize_bson_datetime<S>(date: &Option<DateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(d) => serializer.serialize_str(&d.to_chrono().to_rfc3339()),
        None => serializer.serialize_none(),
    }
}

fn deserialize_bson_datetime<'de, D>(deserializer: D) -> Result<Option<DateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    // Custom deserialization logic can be implemented if necessary
    unimplemented!()
}
