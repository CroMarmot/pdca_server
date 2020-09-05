use actix_web::{web, App, HttpServer};
use log::info;

use crate::dbm::{build_dbm, DBManager};
use std::sync::Mutex;

mod controller;
mod dbm;
mod demo_controller;
mod model;
mod service;

// each ins for each thread
pub struct AppState {
    // service_container: ServiceContainer,
    coll_daily: String,
}

// share and mut between thread
pub struct AppMutState {
    count: Mutex<i32>,
    dbm: Mutex<DBManager>,
}

fn init_logger() {
    use chrono::Local;
    use std::io::Write;

    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
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
    let mut_state = web::Data::new(AppMutState {
        count: Mutex::new(0),
        dbm: Mutex::new(
            build_dbm("mongodb://localhost:27017/", "pdca_v1")
                .await
                .unwrap(),
        ),
    });

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                coll_daily: String::from("pdca_daily"),
            })
            .app_data(mut_state.clone())
            .service(
                web::scope("/api")
                    .route(
                        "/add_daily_pdca",
                        web::post().to(controller::add_daily_pdca),
                    )
                    .route(
                        "/get_daily_pdca",
                        web::post().to(controller::get_daily_pdca),
                    )
                    .service(
                        web::scope("/demo")
                            .route("/", web::get().to(demo_controller::index0))
                            .route("/op1", web::get().to(demo_controller::op1))
                            .route("/name", web::get().to(demo_controller::index_name))
                            .route("/again", web::get().to(demo_controller::again))
                            .route("/dbDemo", web::get().to(demo_controller::db_demo))
                            .route("/sleep_demo", web::get().to(demo_controller::sleep_demo))
                            .route("/custom_resp", web::get().to(demo_controller::custom_resp))
                            .route(
                                "/custom_req/{userid}/{friend}",
                                web::get().to(demo_controller::custom_req),
                            )
                            .route("/custom_json", web::post().to(demo_controller::custom_json))
                            .route("/db_custom", web::post().to(demo_controller::db_custom))
                            .route("/db_query", web::post().to(demo_controller::db_query))
                            .route(
                                "/db_query_one",
                                web::post().to(demo_controller::db_query_one),
                            ),
                    ),
            )
    })
    // .workers(4)
    .bind("0.0.0.0:8088")?
    .shutdown_timeout(60)
    .run()
    .await
}
