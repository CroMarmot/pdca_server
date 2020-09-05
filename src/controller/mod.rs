use crate::model::{DailyKey, DailyPDCA};
use crate::{AppMutState, AppState};
use actix_web::{web, Either, HttpResponse, Responder};
use mongodb::bson::doc;

// fetch("/api/add_daily_pdca",{headers: {'Content-Type': 'application/json'},body:JSON.stringify({
// plan_and_do: [{
// start_time: '2020-09-01',
// end_time: '2020-09-04',
// plan: 'work 1',
// finished: 'not',
// reason: 'hey',
// }],
// check: 'check',
// action: 'action',
// }),method: 'POST'})
// .then((data)=>data.text())
// .then(console.log);
pub async fn add_daily_pdca(
    const_data: web::Data<AppState>,
    data: web::Data<AppMutState>,
    info: web::Json<DailyPDCA>,
) -> impl Responder {
    let mut d_pdca: DailyPDCA = info.into_inner();
    // TODO check first
    //
    // findOneAndUpdate returns a document, updateOne doesn't (it just returns the id if it has created a new document).

    // pub async fn insert_or_update_one<T>(&self, coll_name: &str, filter:Document, sample: T) -> Result<Document>

    d_pdca._id = None; // 阻断id TODO 对多个类型阻断_id

    web::Json(
        data.dbm
            .lock()
            .unwrap()
            // .insert_one_custom(&const_data.coll_daily, d_pdca)
            .insert_or_update_one(
                &const_data.coll_daily,
                doc! {"date":d_pdca.date.clone()},
                d_pdca,
            )
            .await
            .unwrap(),
    )
}
type GetDailyPdcaResult = Either<web::Json<DailyPDCA>, HttpResponse>;

pub async fn get_daily_pdca(
    const_data: web::Data<AppState>,
    data: web::Data<AppMutState>,
    dk: web::Json<DailyKey>,
) -> GetDailyPdcaResult {
    println!("get DailyPDCA");
    let dbm = data.dbm.lock().unwrap();
    if let Some(_id) = &dk._id {
        let res: Option<DailyPDCA> = dbm.find_one(&const_data.coll_daily, doc! {"_id":_id}).await;
        if let Some(d_pdca) = res {
            Either::A(web::Json(d_pdca))
        } else {
            Either::B(HttpResponse::NotFound().body("Not Found daily pdca"))
        }
    } else {
        if let Some(date) = &dk.date {
            let res: Option<DailyPDCA> = dbm
                .find_one(&const_data.coll_daily, doc! {"date":date})
                .await;
            if let Some(d_pdca) = res {
                Either::A(web::Json(d_pdca))
            } else {
                Either::B(HttpResponse::NotFound().body("Not Found daily pdca"))
            }
        } else {
            Either::B(HttpResponse::BadRequest().body("Bad data"))
        }
    }

    // result = data.dbm
    //     .lock()
    //     .unwrap()

    // web::HttpResponse::Ok().body("OK")
}
