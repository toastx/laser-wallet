use actix_web::delete;
use actix_web::get;
use actix_web::post;
use actix_web::put;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use serde_json::json;
use sqlx::query_as;

use crate::models::CreateTodoItem;
use crate::models::TodoModel;
use crate::models::UpdateTodoItem;
use crate::AppState;


#[get("/health_checker")]
pub async fn health_checker_endpoint() -> impl Responder{
    HttpResponse::Ok().json(json!({"status":"success","health":"running"}))
}

#[get("/all")]
pub async fn get_todo_items(data: web::Data<AppState>) -> impl Responder{
    let query_result = query_as!(
        TodoModel,
        "SELECT * FROM todos",
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
    println!("Received request: {:?}", body);
    let query_result = sqlx::query_as!(
        TodoModel,
        "INSERT INTO todos (name,description,completed) values ($1,$2,$3) returning *",
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
                    "message":format!("{}",e)
                }));
        }
    }
    
}

#[get("todos/todo/{id}")]
async fn get_todo_by_id(path: web::Path<uuid::Uuid>, data:web::Data<AppState>)-> impl Responder{
    let todo_id = path.into_inner();
    let query_result = sqlx::query_as!(
        TodoModel,
        "SELECT * FROM todos WHERE id = $1",
        todo_id
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
            return HttpResponse::NotFound()
                .json(json!({
                    "status":"error",
                    "message":"error"
                }));
        }
    }
}


#[put("todos/todo/{id}")]
async fn update_todo(body: web::Json<UpdateTodoItem>,path: web::Path<uuid::Uuid>, data:web::Data<AppState>)-> impl Responder{
    let todo_id = path.into_inner();

    let query_result = sqlx::query_as!(
        TodoModel,
        "UPDATE todos set completed = $2, name = $3, description = $4 WHERE id = $1 RETURNING *",
        todo_id,
        body.completed,
        body.name,
        body.description
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
            return HttpResponse::NotFound()
                .json(json!({
                    "status":"error",
                    "message":"error"
                }));
        }
    }
}

#[delete("todos/todo/{id}")]
async fn delete_todo(path: web::Path<uuid::Uuid>, data:web::Data<AppState>)-> impl Responder{
    let todo_id = path.into_inner();
    let rows_affected = sqlx::query!(
        "Delete from todos where id = $1",
        todo_id
    )
    .execute(&data.pool)
    .await
    .unwrap()
    .rows_affected();


    if rows_affected == 0{
        return HttpResponse::NotFound()
        .json(json!({
            "status":"error",
            "message":"error"
        }))
    }

    HttpResponse::NoContent().finish()
}