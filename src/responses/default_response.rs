use serde::Serialize;
use actix_web::HttpResponse;
use std::io::Result as IOResult;
use std::process::ExitStatus;

#[derive(Serialize)]
pub struct DefaultResponse {
    pub status: String,
    pub message: String,
}

impl DefaultResponse {
    pub fn map_to_http_response(&self) -> HttpResponse {
        match self.status.as_str() {
            "Ok" => HttpResponse::Ok().json(self),
            "Error" => HttpResponse::BadRequest().json(self),
            "InternalError" => HttpResponse::InternalServerError().json(self),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }

    pub fn from_result_exit_status(result: IOResult<ExitStatus>, success_message: &str) -> DefaultResponse {
        match result {
            Err(e) => DefaultResponse {
                status: "InternalError".to_string(),
                message: e.to_string(),
            },
            Ok(r) => match r.success() {
                true => DefaultResponse {
                    status: "Ok".to_string(),
                    message: success_message.to_string(),
                },
                false => DefaultResponse {
                    status: "InternalError".to_string(),
                    message: r.to_string(),
                }
            }
        }
    }
}