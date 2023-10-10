use std::collections::HashMap;
use std::error::Error;
use crate::blobstore::{BlobStore, ID};
use serde::{Serialize, de::DeserializeOwned};

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

impl<'a> BlobStore<'a> for MapStore {
    type RawObject = String;

    fn get<T: DeserializeOwned>(&'a self, id: &ID, process: fn(&'a Self::RawObject) -> Result<T, Box<dyn Error>>) -> Result<T, Box<dyn Error>> {
        if let Some(d) = self.data.get(id) {
            Ok(process(d.into()).unwrap())
        } else {
            Err(Box::<dyn Error>::from("No such key"))
        }
    }

    fn put<T: Serialize>(&mut self, id: &ID, t: &T, process: fn(&T) -> Result<Self::RawObject, Box<dyn Error>>) -> Result<(), Box<dyn Error>> {
        let data = process(t)?;
        self.put_object(id, &data)
    }


    fn get_object(&'a self, id: &ID) -> Result<&'a Self::RawObject, Box<dyn Error>> {
        if let Some(d) = self.data.get(id) {
            Ok(d)
        } else {
            Err(Box::<dyn Error>::from("No such key"))
        }
    }

    fn put_object(&mut self, id: &ID, obj: &Self::RawObject) -> Result<(), Box<dyn Error>> {
        self.data.insert(id.clone(), obj.clone());
        Ok(())
    }
}


