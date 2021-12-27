use actix_web::{web, HttpResponse, Responder};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio;

use crate::AppMutState;

// 返回的
#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

// http://127.0.0.1:8088/api/demo/
pub async fn index0() -> impl Responder {
    println!("index0");
    HttpResponse::Ok().body("Hello world!")
}

pub async fn op1(data: web::Data<AppMutState>) -> impl Responder {
    let count = data.count.lock().unwrap();
    format!("op 1 {}!", count)
}

pub async fn index_name(data: web::Data<AppMutState>) -> String {
    let mut count = data.count.lock().unwrap();
    *count += 1;
    format!("Hello {}!", count)
}
pub async fn sleep_demo() -> impl Responder {
    // std::thread::sleep(Duration::from_secs(5)); // <-- Bad practice!!! Will cause the current worker thread to hang!
    tokio::time::delay_for(Duration::from_secs(5)).await; // <-- Ok. Worker thread will handle other requests here
    "response"
}

pub async fn again() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

pub async fn db_demo(data: web::Data<AppMutState>) -> impl Responder {
    let dbm = data.dbm.lock().unwrap();
    dbm.insert_one("some-coll").await.unwrap();

    HttpResponse::Ok().body("Hello world again!")
}

// fetch("/api/demo/custom_resp",{headers: {'Content-Type': 'application/json'},method: 'GET'}).then((data)=>data.text()).then(console.log);
pub async fn custom_resp() -> impl Responder {
    web::Json(MyObj { name: "user" })
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CustJson {
    // https://serde.rs/field-attrs.html
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    username: String,
}

// fetch("/api/demo/custom_json",{headers: {'Content-Type': 'application/json'},method: 'POST',body:JSON.stringify({username:'?'})}).then((data)=>data.text()).then(console.log);
// 请求自定义json
pub async fn custom_json(info: web::Json<CustJson>) -> actix_web::Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

// TODO db update 和 类型不匹配处理
pub async fn db_custom(data: web::Data<AppMutState>, info: web::Json<CustJson>) -> impl Responder {
    let dbm = data.dbm.lock().unwrap();
    let mut info_cust: CustJson = info.into_inner();
    info_cust.id = None; // 阻断id TODO 任何类型
                         // dbm.insert_custom("some-coll", info_cust).await.unwrap();
    dbm.insert_one_custom("some-coll", info_cust).await.unwrap();

    HttpResponse::Ok().body("ok")
}

// 查询
pub async fn db_query(data: web::Data<AppMutState>) -> impl Responder {
    let dbm = data.dbm.lock().unwrap();
    println!("db_query");

    let res: Vec<CustJson> = dbm.find_data("some-coll").await;

    println!("{:#?}", res);
    // HttpResponse::Ok().body("ok")
    web::Json(res)
}

// 查询
pub async fn db_query_one(data: web::Data<AppMutState>) -> impl Responder {
    let dbm = data.dbm.lock().unwrap();
    println!("db_query");

    let res: Option<CustJson> = dbm.find_one("some-coll", doc! { "date":"2020-09-05"}).await;

    println!("{:#?}", res);
    // HttpResponse::Ok().body("ok")
    web::Json(res)
}

// TODO document 增加 id
