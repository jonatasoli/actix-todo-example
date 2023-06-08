use actix_web::dev::Service;
use actix_web::{ App, test};
use actix_todo_example::health_check;

#[actix_rt::test]
async fn test_health_check() {
    let app = test::init_service(
        App::new().service(health_check)
    )
    .await;

    let req = test::TestRequest::get().uri("/health_check").to_request();
    let resp = app.call(req).await.unwrap();

    assert!(resp.status().is_success());
}
