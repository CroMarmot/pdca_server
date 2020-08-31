use futures::Future;

use mongodb::{
    bson::{doc, Document},
    error::Error,
    options::ClientOptions,
    results::InsertOneResult,
    Client, Collection,
};

#[derive(Clone)]
pub struct UserService {
    collection: Collection,
}

impl UserService {
    pub fn new(collection: Collection) -> UserService {
        UserService { collection }
    }

    // pub fn create(&self, name: &str) -> Result<InsertOneResult, Error> {
    //     self.collection.insert_one(doc! {"name": name}, None)
    // }

    // pub fn get(&self) -> Result<Option<Document>, Error> {
    //     self.collection.find_one(doc! {}, None)
    // }
}
