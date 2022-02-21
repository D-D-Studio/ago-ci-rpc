use actix_web::{Responder, web};
use uuid::Uuid;
use crate::responses::default_response::{DefaultResponse, ResponseStatus};
use crate::utils::{directory_exists, file_exists, run_command_with_path};

pub async fn deploy(id: web::Path<String>) -> impl Responder {
    let request_id = Uuid::new_v4();
    let container_id = id.into_inner();

    let dir_path = format!("./config/{}", container_id.as_str());
    let file_path = format!("{}/service.yml", dir_path);

    if !directory_exists(dir_path.clone()).await || !file_exists(file_path).await {
        let response = DefaultResponse {
            status: ResponseStatus::Error,
            message: "Container not found!".to_string(),
        };

        return response.map_to_http_response();
    }

    println!("[{}] Start deployment for container {}", request_id, container_id);

    let result = run_command_with_path(
        "docker-compose --f service.yml up --build".to_string(),
        dir_path,
    ).await;

    let response = DefaultResponse::from_result_exit_status(
        result,
        "Container deployed!",
    );

    println!("[{}] End deployment for container {}", request_id, container_id);

    response.map_to_http_response()
}