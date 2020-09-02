use futures::stream::StreamExt;
use mongodb::{
    bson::{from_bson, doc,Bson, Document,to_bson},
    error::Error,
    results::InsertOneResult,
    Collection,
    options::ClientOptions,
    Database,
    Client,
    Cursor
};
use crate::controller::CustJson;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct DBManager {
    db: Database,
}
// fn decode<'a,T>(doc: &Document) -> Option<T>
//     where T: Deserialize<'a>
// {
//     from_bson(Bson::Document(doc.clone())).ok()
// }
//
// fn encode<T: Serialize>(doc: &T) -> Option<Document> {
//     match to_bson(doc) {
//         Ok(Bson::Document(d)) => Some(d),
//         _ => None,
//     }
// }

impl DBManager{
    pub async fn insert_one(&self, coll_name:&str) -> Result<InsertOneResult,Error>{
        // Parse a connection string into an options struct.
        let coll = self.db.collection(coll_name);
        let result = coll.insert_one(doc! { "x": 1 }, None).await?;
        println!("{:#?}", result);
        Ok(result)
    }

    pub async fn insert_custom<T>(&self, coll_name:&str, cust_item: T) -> Result<InsertOneResult,Error>
        where T:Serialize
    {
        // Parse a connection string into an options struct.
        let coll = self.db.collection(coll_name);
        let result = coll.insert_one( doc!{"value":to_bson(&cust_item).unwrap()}, None).await?;
        println!("{:#?}", result);
        Ok(result)
    }

    pub async fn find_data(&self, coll_name:&str) -> Result<Vec<CustJson>,Error>{
        let coll = self.db.collection(coll_name);
        let mut cursor = coll.find(None,None).await?;
        while let Some(doc) = cursor.next().await {
            println!("{}", doc?)
        }
        let results: Vec<Result<Document,Error>> = cursor.collect().await;
        println!("{:#?}", results);
        let outputs: Vec<CustJson> = results.iter()
            .map(|r_d| from_bson(Bson::Document(r_d.clone())))
            .collect();
        println!("{:#?}", outputs);
        Ok(outputs)
    }
}

pub async fn build_dbm(db_name:&str) -> Result<DBManager,Error> {
    let client = Client::with_uri_str("mongodb://localhost:27017/").await?;

    // println!("DBs:");
    // // // List the names of the databases in that deployment.
    // let db_names = client.list_database_names(None,None).await?;
    // for db_name in db_names {
    //     println!("\t{}", db_name);
    // }

    // println!("Colections:");
    let db = client.database(db_name);
    // for coll_name in db.list_collection_names(None).await? {
    //     println!("\t{}", coll_name);
    // }

    Ok(DBManager { db })
}
