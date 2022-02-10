use actix_web::{Responder};
use uuid::Uuid;
use crate::responses::default_response::DefaultResponse;
use crate::utils::{directory_exists, run_command};

pub async fn update() -> impl Responder {
    let request_id = Uuid::new_v4();
    let cmd = match directory_exists("./config".to_string()).await {
        true => "cd ./config && git pull".to_string(),
        false => format!("git clone {} ./config", env!("DOCKER_COMPOSE_REPO")),
    };

    println!("[{}] Start updating docker-compose files", request_id);

    let result = run_command(cmd).await;
    let response = DefaultResponse::from_result_exit_status(
        result,
        "Files updated!",
    );

    println!("[{}] End updating docker-compose files", request_id);

    response.map_to_http_response()
}