use actix_web::{test, App};
use actix_todo_example::{health_check, get_task, list_tasks, create_task, update_task};
use actix_todo_example::{NewTask, Task};
use serde_json::json;

#[actix_rt::test]
async fn test_get_task() {
    let app = test::init_service(App::new().service(get_task)).await;

    // Realize uma chamada GET para a rota /task/{id} com um ID válido
    let id = 1;
    let request = test::TestRequest::get().uri(&format!("/task/{}", id.to_string())).to_request();
    let response = test::call_service(&app, request).await;

    // Verifique se a resposta tem status 200 OK
    assert!(response.status().is_success());

    // Verifique se a resposta contém os dados da tarefa corretos
    let body = test::read_body(response).await;
    let expected_task = Task {
        id: id,
        name: "Hello".to_string(),
        completed: false,
    };
    let expected_body = serde_json::to_string(&expected_task).unwrap();
    assert_eq!(body, expected_body);
}

#[actix_rt::test]
async fn test_list_tasks() {
    let app = test::init_service(App::new().service(list_tasks)).await;

    // Realize uma chamada GET para a rota /tasks
    let request = test::TestRequest::get().uri("/tasks").to_request();
    let response = test::call_service(&app, request).await;

    // Verifique se a resposta tem status 200 OK
    assert!(response.status().is_success());

    // Verifique se a resposta contém a lista de tarefas correta
    let body = test::read_body(response).await;
    let mut expected_tasks: Vec<Task> = Vec::new();
    for i in 0..10 {
        let todo = Task {
            id: i,
            name: format!("Task {}", i),
            completed: false,
        };
        expected_tasks.push(todo);
    }
    let expected_body = serde_json::to_string(&expected_tasks).unwrap();
    assert_eq!(body, expected_body);
}

#[actix_rt::test]
async fn test_create_task() {
    let app = test::init_service(App::new().service(create_task)).await;

    // Crie uma nova tarefa
    let new_task = NewTask {
        name: "New Task".to_string(),
        completed: false,
    };
    let request = test::TestRequest::post()
        .uri("/task")
        .set_json(&new_task)
        .to_request();
    let response = test::call_service(&app, request).await;

    // Verifique se a resposta tem status 201 Created
    assert_eq!(response.status(), actix_web::http::StatusCode::CREATED);

    // Verifique se a resposta contém a nova tarefa criada
    let body = test::read_body(response).await;
    let expected_body = serde_json::to_string(&new_task).unwrap();
    assert_eq!(body, expected_body);
}

#[actix_rt::test]
async fn test_update_task() {
    let app = test::init_service(App::new().service(update_task)).await;

    // Atualize uma tarefa existente
    let id = 1;
    let update_data = json!({
        "name": "Updated Task",
        "completed": true
    });
    let request = test::TestRequest::put()
        .uri(&format!("/task/{}", id.to_string()))
        .set_json(&update_data)
        .to_request();
    let response = test::call_service(&app, request).await;

    // Verifique se a resposta tem status 204 No Content
    assert_eq!(response.status(), actix_web::http::StatusCode::NO_CONTENT);
}
