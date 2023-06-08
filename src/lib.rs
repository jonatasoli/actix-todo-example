use actix_web::{get, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewTask {
    pub name: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTask {
    pub name: Option<String>,
    pub completed: Option<bool>,
}


#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[get("/task/{id}")]
async fn get_task(id: web::Path<String>) -> impl Responder {
    let todo_id: u32 = match id.parse() {
        Ok(parsed_id) => parsed_id,
        Err(_) => {
            return HttpResponse::BadRequest().body(format!("The ID '{}' has invalid format", id));
        }
    };
    let todo = Task {
        id: todo_id,
        name: "Hello".to_string(),
        completed: false,
    };
    HttpResponse::Ok().json(todo)
}


#[get("/tasks")]
async fn list_tasks() -> impl Responder {
    let mut todos: Vec<Task> = Vec::new();
    for i in 0..10 {
        let todo = Task {
            id: i,
            name: format!("Task {}", i),
            completed: false,
        };
        todos.push(todo);
    }

    HttpResponse::Ok().json(todos)
}


#[post("/task")]
async fn create_task(req_body: String) -> impl Responder {
    match convert_req_body_to_new_task(req_body).await {
        Ok(new_task) => HttpResponse::Created().json(new_task),
        Err(err) => err,
    }
}


#[put("/task/{id}")]
async fn update_task(id: web::Path<String>, req_body: String) -> impl Responder {
    println!("req_body: {}" , req_body);
    match convert_id(id.into_inner()).await {
        Ok(todo_id) => {
            println!("Updating task with id {}", todo_id);
            HttpResponse::NoContent().finish()
        }
        Err(err) => err,
    }
}

async fn convert_id(id: String) -> Result<u32, HttpResponse> {
    match id.parse() {
        Ok(parsed_id) => Ok(parsed_id),
        Err(_) => Err(HttpResponse::BadRequest().body(format!("The ID '{}' has invalid format", id)))
    }
}

async fn convert_req_body_to_new_task(req_body: String) -> Result<NewTask, HttpResponse> {
    match serde_json::from_str::<NewTask>(&req_body) {
        Ok(new_task) => Ok(new_task),
        Err(_) => Err(HttpResponse::BadRequest().body("Invalid request body")),
    }
}


// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(get_task)
            .service(list_tasks)
            .service(create_task)
            .service(update_task)
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
