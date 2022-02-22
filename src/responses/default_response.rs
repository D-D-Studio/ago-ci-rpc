use serde::Serialize;
use actix_web::HttpResponse;
use std::io::Result as IOResult;
use std::process::ExitStatus;

#[derive(Serialize)]
pub enum ResponseStatus {
    Ok,
    Error,
    InternalError,
}

#[derive(Serialize)]
pub struct DefaultResponse {
    pub status: ResponseStatus,
    pub message: String,
}

impl DefaultResponse {
    pub fn map_to_http_response(&self) -> HttpResponse {
        match self.status {
            ResponseStatus::Ok => HttpResponse::Ok().json(self),
            ResponseStatus::Error => HttpResponse::BadRequest().json(self),
            ResponseStatus::InternalError => HttpResponse::InternalServerError().json(self),
        }
    }

    pub fn from_result_exit_status(result: IOResult<ExitStatus>, success_message: &str) -> DefaultResponse {
        match result {
            Err(e) => DefaultResponse {
                status: ResponseStatus::InternalError,
                message: e.to_string(),
            },
            Ok(r) => if r.success() {
                DefaultResponse {
                    status: ResponseStatus::Ok,
                    message: success_message.to_string(),
                }
            } else {
                DefaultResponse {
                    status: ResponseStatus::InternalError,
                    message: r.to_string(),
                }
            }
        }
    }
}