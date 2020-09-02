use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize,Deserialize};
use log::info;

use service::UserService;
use std::sync::Mutex;

mod controller;
mod service;
mod dbm;

pub struct ServiceContainer {
    user: UserService,
}

impl ServiceContainer {
    pub fn new(user: UserService) -> Self {
        ServiceContainer { user }
    }
}

// each ins for each thread
pub struct AppState {
    // service_container: ServiceContainer,
    count: Mutex<i32>,
}

// share and mut between thread
pub struct AppMutState {
    // service_container: ServiceContainer,
    count: Mutex<i32>,
}

fn init_logger() {
    use chrono::Local;
    use std::io::Write;

    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    // 设置日志打印格式
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.module_path().unwrap_or("<unnamed>"),
                &record.args()
            )
        })
        .init();
    info!("env_logger initialized.");
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    // Http server constructs an application instance for each thread
    let MutState = web::Data::new(AppMutState {
        count: Mutex::new(0),
    });


    HttpServer::new(move || {
        App::new()
            // static
        //    .data(AppState {
        //        count: Mutex::new(0)
        //}) // static
            .app_data(MutState.clone())
            .route("/", web::get().to(controller::index0))
            .route("/name", web::get().to(controller::index_name))
            .route("/again", web::get().to(controller::again))
            .route("/dbDemo", web::get().to(controller::db_demo))
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}
