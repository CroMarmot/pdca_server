use futures::stream::StreamExt;
use std::fmt::Debug;
use mongodb::{
    bson::{from_bson, doc,Bson, Document,to_bson},
    error::{Error,Result},
    results::InsertOneResult,
    Collection,
    options::ClientOptions,
    Database,
    Client,
    Cursor
};
use crate::controller::CustJson;
use serde::{Serialize, Deserialize,de::DeserializeOwned};

// https://docs.rs/mongodm/0.4.2/src/mongodm/repository.rs.html
fn h_model_to_doc<T>(model: &T) -> Result<Document>
    where T:Serialize
{
    let bson = to_bson(&model)?;
    if let Bson::Document(doc) = bson {
        Ok(doc)
    } else {
        Err(mongodb::error::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "model can't be serialized into a `Bson::Document`",
        )))
    }
}


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
    pub async fn insert_one(&self, coll_name:&str) -> Result<InsertOneResult>{
        // Parse a connection string into an options struct.
        let coll = self.db.collection(coll_name);
        let result = coll.insert_one(doc! { "x": 1 }, None).await?;
        println!("{:#?}", result);
        Ok(result)
    }

    pub async fn insert_custom<T>(&self, coll_name:&str, cust_item: T) -> Result<InsertOneResult>
        where T:Serialize
    {
        // Parse a connection string into an options struct.
        let coll:Collection = self.db.collection(coll_name);
        let result = coll.insert_one( doc!{"value":to_bson(&cust_item).unwrap()}, None).await?;
        println!("{:#?}", result);
        Ok(result)
    }


// 插入不对称 多个 value
     pub async fn insert_one_custom<T>(&self, coll_name:&str , sample: T) -> Result<InsertOneResult>
         where T:Serialize
     {
         let coll:Collection = self.db.collection(coll_name);
         let result = coll.insert_one(h_model_to_doc(&sample)?,None).await?;

         // let result = coll.insert_one( doc!{"value":to_bson(&cust_item).unwrap()}, None).await?;
         println!("{:#?}", result);
         Ok(result)
     }

    // https://github.com/mongodb/mongo-rust-driver/blob/master/src/cursor/mod.rs#L45-L60
    pub async fn find_data<T >(&self, coll_name:&str) -> Vec<T>
    where T:DeserializeOwned
    {
        let coll = self.db.collection(coll_name);
        // let mut cursor = coll.find(None,None).await.unwrap();
        // while let Some(doc) = cursor.next().await {
        //     println!("{}", doc.unwrap())
        // }
        // let results: Vec<Result<Document>> = cursor.collect().await;
        // println!("{:#?}", results);

        coll.find(None, None)
            .await
            .unwrap()
            .filter_map(|item| async move { item.ok() })
            .map(|doc| from_bson(Bson::Document(doc)).expect("Decode error"))
            .collect()
            .await

//         println!("{:#?}", outputs);
//         Ok(outputs)
    }
}

pub async fn build_dbm(db_name:&str) -> Result<DBManager> {
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
