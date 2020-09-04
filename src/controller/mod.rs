use actix_web::{get, web, HttpResponse, Responder, HttpRequest,Error};
use std::sync::{Mutex, Arc};
use mongodb::{bson::oid::ObjectId};
use serde::{Serialize,Deserialize};
use std::time::Duration;
use tokio;

use super::dbm::build_dbm;
use crate::{AppState, AppMutState};
use futures::future::{ready, Ready};
use crate::dbm::DBManager;
use std::future::Future;

// 返回的
#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

#[derive(Deserialize)]
pub struct User {
    name: String,
}

pub async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn index0() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn op1(data: web::Data<AppMutState>) -> impl Responder{
    let mut count =    data.count.lock().unwrap();
    format!("op 1 {}!",count)
}

pub async fn index_name(data: web::Data<AppMutState>) -> String {
    let mut count =    data.count.lock().unwrap();
    *count+=1;
    format!("Hello {}!",count)
}
pub async fn sleep_demo() -> impl Responder {
    // std::thread::sleep(Duration::from_secs(5)); // <-- Bad practice!!! Will cause the current worker thread to hang!
    tokio::time::delay_for(Duration::from_secs(5)).await; // <-- Ok. Worker thread will handle other requests here
    "response"
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


// MyObj 支持 JSON 返回
// Responder
// TODO 任意结构返回json
// impl Responder for MyObj{
//     type Error = Error;
//     type Future = Ready<Result<HttpResponse, Error>>;
//
//     fn respond_to(self, _req: &HttpRequest) -> Self::Future {
//         let body = serde_json::to_string(&self).unwrap();
//         // Create response and set content type
//         ready(Ok(HttpResponse::Ok()
//             .content_type("application/json")
//             .body(body)))
//     }
// }

// impl<T> Responder for T
//     where T: Serialize
// {
//     type Error = Error;
//     type Future = Ready<Result<HttpResponse, Error>>;
//
//     fn respond_to(self, _req: &HttpRequest) -> Self::Future {
//         let body = serde_json::to_string(&self).unwrap();
//         // Create response and set content type
//         ready(Ok(HttpResponse::Ok()
//             .content_type("application/json")
//             .body(body)))
//     }
// }


// fetch("/api/custom_resp",{headers: {'Content-Type': 'application/json'},method: 'GET'}).then((data)=>data.text()).then(console.log);
pub async fn custom_resp() -> impl Responder {
   web::Json( MyObj { name: "user" } )
}


#[derive(Deserialize)]
pub struct Info {
    userid: u32,
    friend: String,
}

// 自定义路径
// extract path info using serde
pub async fn custom_req(info: web::Path<Info>) -> actix_web::Result<String> {
    Ok(format!("Welcome {}, userid {}!", info.friend, info.userid))
}

#[derive(Serialize,Deserialize,Debug)]
pub struct CustJson{
    // https://serde.rs/field-attrs.html
    #[serde(rename = "_id",skip_serializing)]
    id: Option<ObjectId>,
    username: String,
}

// fetch("/api/custom_json",{headers: {'Content-Type': 'application/json'},method: 'POST',body:JSON.stringify({username:'?'})}).then((data)=>data.text()).then(console.log);
// 请求自定义json
pub async fn custom_json(info: web::Json<CustJson>) -> actix_web::Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

// TODO db update 和 类型不匹配处理
pub async fn db_custom(info: web::Json<CustJson>) -> impl Responder{
    let dbm:DBManager = build_dbm("pdca_v1").await.unwrap();
    let info_cust:CustJson = info.into_inner();
    // dbm.insert_custom("some-coll", info_cust).await.unwrap();
    dbm.insert_one_custom("some-coll", info_cust).await.unwrap();

    HttpResponse::Ok().body("ok")
}

// 查询
pub async fn db_query() -> impl Responder{
    let dbm:DBManager = build_dbm("pdca_v1").await.unwrap();
    println!("db_query");

    let res:Vec<CustJson> = dbm.find_data("some-coll").await;

    println!("{:#?}", res);
    // HttpResponse::Ok().body("ok")
    web::Json(res)
}

// TODO document 增加 id
