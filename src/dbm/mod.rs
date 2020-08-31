use mongodb::{
    bson::{doc, Document},
    error::Error,
    results::InsertOneResult,
    Collection,
    options::ClientOptions,
    Client,
};

pub struct DBManager {
}

impl DBManager{
    pub async fn insert_one(&self) -> Result<InsertOneResult,Error>{
        // Parse a connection string into an options struct.
        let client = Client::with_uri_str("mongodb://localhost:27017/").await?;

        println!("!!");
        let db = client.database("some_db");
        for coll_name in db.list_collection_names(None).await? {
            println!("collection: {}", coll_name);
        }

        let coll = db.collection("some-coll");
        let result = coll.insert_one(doc! { "x": 1 }, None).await?;
        println!("{:#?}", result);

        Ok(result)
    }
    pub fn hey(&self){
        println!("hey func");
    }
}

pub async fn build_dbm() -> Result<DBManager,Error> {
    println!("build");
    // Get a handle to the deployment.
    let client = Client::with_uri_str("mongodb://localhost:27017/").await?;

    println!("!!");
    // // List the names of the databases in that deployment.
    let db_names = client.list_database_names(None,None).await?;
    for db_name in db_names {
        println!("{}", db_name);
    }
    println!("??");

    Ok(DBManager { })
}
