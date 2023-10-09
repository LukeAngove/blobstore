use std::collections::HashMap;
use std::error::Error;
use crate::blobstore::{BlobStore, ID};

pub struct MapStore {
    data: HashMap<ID, String>,
}

impl MapStore {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl BlobStore for MapStore {
    type RawObject = String;

    fn get_object<T>(&self, id: &ID, process: fn(&Self::RawObject) -> Result<T, Box<dyn Error>>) -> Result<T, Box<dyn Error>> {
        if let Some(d) = self.data.get(id) {
            Ok(process(d.into()).unwrap())
        } else {
            Err(Box::<dyn Error>::from("No such key"))
        }
    }

    fn put_object(&mut self, id: &ID, obj: &Self::RawObject) -> Result<(), Box<dyn Error>> {
        self.data.insert(id.clone(), obj.clone());
        Ok(())
    }
}


