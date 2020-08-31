// use actix_web::{error, HttpResponse};
// use failure::Fail;
// use serde::{Deserialize, Serialize};
//
// #[derive(Fail, Debug)]
// pub enum BusinessError {
//     #[fail(display = "Validation error on field: {}", field)]
//     ValidationError { field: String },
//     #[fail(display = "An internal error occurred. Please try again later.")]
//     InternalError,
// }
//
// impl error::ResponseError for BusinessError {
//     fn error_response(&self) -> HttpResponse {
//         match *self {
//             BusinessError::ValidationError { .. } => {
//                 let resp = Resp::err(10001, &self.to_string());
//                 HttpResponse::BadRequest().json(resp)
//             }
//             _ => {
//                 let resp = Resp::err(10000, &self.to_string());
//                 HttpResponse::InternalServerError().json(resp)
//             }
//         }
//     }
//     // 重写response的序列化结果
//     fn render_response(&self) -> HttpResponse {
//         self.error_response()
//     }
// }
//
// #[derive(Deserialize, Serialize)]
// pub struct Resp<T>
// where
//     T: Serialize,
// {
//     code: i32,
//     message: String,
//     data: Option<T>,
// }
//
// impl<T: Serialize> Resp<T> {
//     pub fn ok(data: T) -> Self {
//         Resp {
//             code: 0,
//             message: "ok".to_owned(),
//             data: Some(data),
//         }
//     }
//
//     pub fn to_json_result(&self) -> Result<HttpResponse, BusinessError> {
//         Ok(HttpResponse::Ok().json(self))
//     }
// }
//
// impl Resp<()> {
//     pub fn err(error: i32, message: &str) -> Self {
//         Resp {
//             code: error,
//             message: message.to_owned(),
//             data: None,
//         }
//     }
// }
