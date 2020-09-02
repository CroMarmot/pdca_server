use actix_web::{get, web, HttpResponse, Responder};
use std::sync::{Mutex, Arc};
use serde::Deserialize;

use super::dbm::build_dbm;
use crate::{AppState, AppMutState};

#[derive(Deserialize)]
pub struct User {
    name: String,
}

#[get("/hello")]
pub async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn index0() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn index_name(data: web::Data<AppMutState>) -> String {
    let mut count =    data.count.lock().unwrap();
    *count+=1;
    format!("Hello {}!",count) // <- response with app_name
}

pub async fn again() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

pub async fn db_demo() -> impl Responder{
    let dbm = build_dbm("pdca_v1").await.unwrap();
    dbm.insert_one("some-coll").await.unwrap();

    HttpResponse::Ok().body("Hello world again!")
}

// pub async fn index(
//     app_data: web::Data<crate::AppState>,
//     user: web::Query<User>,
// ) -> impl Responder {
//     let result = web::block(move || app_data.service_container.user.create(&user.name)).await;
//     match result {
//         Ok(result) => HttpResponse::Ok().json(result.inserted_id),
//         Err(e) => {
//             println!("Error while getting, {:?}", e);
//             HttpResponse::InternalServerError().finish()
//         }
//     }
// }
//
// pub async fn get(
//     app_data: web::Data<crate::AppState>,
// ) -> impl Responder {
//     let result = web::block(move || app_data.service_container.user.get()).await;
//     match result {
//         Ok(result) => HttpResponse::Ok().json(result),
//         Err(e) => {
//             println!("Error while getting, {:?}", e);
//             HttpResponse::InternalServerError().finish()
//         }
//     }
// }