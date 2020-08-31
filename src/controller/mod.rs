use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    name: String,
}

pub async fn index0() -> impl Responder {
    // let client = Client::with_uri_str("mongodb://localhost:27017/").await?;
    // let db = client.database("some_db");
    // for coll_name in db.list_collection_names(None).await? {
    //     println!("collection: {}", coll_name);
    // }

    // let coll = db.collection("some-coll");
    // let result = coll.insert_one(doc! { "x": 1 }, None).await?;
    // println!("{:#?}", result);
    HttpResponse::Ok().body("Hello world!")
}

pub async fn again() -> impl Responder {
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