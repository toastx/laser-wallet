use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono;



#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct TodoModel{
    pub id: Uuid,
    pub name: String,
    pub description:String,
    pub completed:bool,
    pub created_at:Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct CreateTodoItem{
    pub name: String,
    pub description: String,
    pub completed:bool,
}

#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct UpdateTodoItem{
    pub id: Uuid,
    pub completed: bool
}

#[derive(Debug,FromRow,Deserialize,Serialize)]
pub struct DeleteTodoItem{
    pub id: Uuid,
}


