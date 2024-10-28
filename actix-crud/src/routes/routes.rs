use actix_web::get;
use actix_web::post;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use serde_json::json;
use sqlx::query_as;

use crate::models::models::CreateTodoItem;
use crate::AppState;

#[get("/")]
pub async fn hello_world() -> impl Responder {
    HttpResponse::Ok().json(json!({"status":"sucess","message":"hello world!!"}))
}

#[get("/toastx")]
pub async fn toastx() -> impl Responder {
    HttpResponse::Ok().json(json!({"status":"sucess","message":"toastx website"}))
}

#[get("/health_checker")]
pub async fn health_checker_endpoint() -> impl Responder{
    HttpResponse::Ok().json(json!({"status":"success","health":"running"}))
}

#[get("/all")]
pub async fn get_todo_items(data: web::Data<AppState>) -> impl Responder{
    let query_result = query_as!(
        TodoModel,
        "SELECT * FROM todos"
    )
    .fetch_all(&data.pool)
    .await;

    if query_result.is_err(){
        let error_message = "Some error while fetching todos";
        HttpResponse::InternalServerError().json(json!({"status":"error","message":error_message}))
    }
    else    {   
    let fields = query_result.unwrap();
    let json_response = json!({
        "status":"success",
        "no of todos":fields.len(),
        "todos": fields
    });

    HttpResponse::Ok()
        .json(json_response)
    }
}

#[post("/todos/todo")]
async fn create_todo(body: web::Json<CreateTodoItem>, data: web::Data<AppState>)-> impl Responder{
    let query_result = sqlx::query_as!(
        TodoModel,
        "INSERT INTO todos (name,description,completed) value ($1,$2,$3) returning *",
        body.name.to_string(),
        body.description.to_string(),
        body.completed
    )
    .fetch_one(&data.pool)
    .await;

    match query_result{
        Ok(todo) =>{
            let todo_response = json!({
                "status":"success", 
                "data": json!({
                    "todo":todo
                })
            });
            return HttpResponse::Ok()
                .json(todo_response);
        }
        Err(e) =>{
            return HttpResponse::InternalServerError()
                .json(json!({
                    "status":"error",
                    "message":"error"
                }));
        }
    }
    
}

