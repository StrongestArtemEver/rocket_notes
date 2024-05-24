use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket::serde::Serialize;
use std::io::Cursor;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error: String,
}

impl<'r> Responder<'r, 'static> for ErrorResponse {
    fn respond_to(self, _: &'r Request<'_>) -> Result<Response<'static>, Status> {
        let body = serde_json::to_string(&self).unwrap();  // Преобразуем ошибку в строку JSON
        Response::build()
            .header(rocket::http::ContentType::JSON)
            .status(Status::InternalServerError)
            .sized_body(body.len(), Cursor::new(body))
            .ok()
    }
}

impl From<sqlx::Error> for ErrorResponse {
    fn from(error: sqlx::Error) -> Self {
        ErrorResponse {
            error: error.to_string(),
        }
    }
}
