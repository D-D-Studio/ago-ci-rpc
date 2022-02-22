use actix_web::{Responder, web};
use serde_derive::{Deserialize};
use uuid::Uuid;
use crate::responses::default_response::{DefaultResponse, ResponseStatus};
use crate::utils::{directory_exists, file_exists, run_command_with_path_and_env};

#[derive(Deserialize)]
pub struct DeployRequest {
    id: String,
    tag: String,
}

pub async fn deploy(request: web::Path<DeployRequest>) -> impl Responder {
    let uuid = Uuid::new_v4();
    let dir_path = format!("./config/{}", request.id.as_str());
    let file_path = format!("{}/service.yml", dir_path);

    if !directory_exists(dir_path.clone()).await || !file_exists(file_path).await {
        let response = DefaultResponse {
            status: ResponseStatus::Error,
            message: "Container not found!".to_string(),
        };

        return response.map_to_http_response();
    }

    println!("[{}] Start deployment for container {} with tag {}", uuid, request.id, request.tag);

    let result = run_command_with_path_and_env(
        "docker-compose --f service.yml up --build".to_string(),
        dir_path,
        vec![("CI_TAG".to_string(), request.tag.clone())],
    ).await;

    let response = DefaultResponse::from_result_exit_status(
        result,
        "Container deployed!",
    );

    println!("[{}] End deployment for container {} with tag {}", uuid, request.id, request.tag);

    response.map_to_http_response()
}