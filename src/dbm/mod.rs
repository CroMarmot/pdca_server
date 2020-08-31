use mongodb::{
    bson::{doc, Document},
    error::Error,
    results::InsertOneResult,
    Collection,
    options::ClientOptions,
    Database,
    Client,
};

pub struct DBManager {
    db: Database,
}

impl DBManager{
    pub async fn insert_one(&self, coll_name:&str) -> Result<InsertOneResult,Error>{
        // Parse a connection string into an options struct.
        let coll = self.db.collection(coll_name);
        let result = coll.insert_one(doc! { "x": 1 }, None).await?;
        println!("{:#?}", result);
        Ok(result)
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
