use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize,Deserialize};
use log::info;

use service::UserService;

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

pub struct AppState {
    service_container: ServiceContainer,
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
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(controller::index0))
            .route("/again", web::get().to(controller::again))
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}
