use crate::blobstore::BlobStore;
use crate::node::HasID;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;

pub struct MapStore<ID: Eq + PartialEq + Hash + Clone> {
    data: HashMap<ID, String>,
}

impl<ID: Eq + PartialEq + Hash + Clone> MapStore<ID> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl<'a, ID: Eq + PartialEq + Hash + Clone> BlobStore<'a> for MapStore<ID> {
    type RawObject = String;

    fn get<T: DeserializeOwned>(
        &'a self,
        id: &Self::ID,
        process: fn(&'a Self::RawObject) -> Result<T, Box<dyn Error>>,
    ) -> Result<T, Box<dyn Error>> {
        if let Some(d) = self.data.get(id) {
            Ok(process(d.into()).unwrap())
        } else {
            Err(Box::<dyn Error>::from("No such key"))
        }
    }

    fn put<T: Serialize>(
        &mut self,
        id: &ID,
        t: &T,
        process: fn(&T) -> Result<Self::RawObject, Box<dyn Error>>,
    ) -> Result<(), Box<dyn Error>> {
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

impl<'a, ID: Eq + PartialEq + Hash + Clone> HasID for MapStore<ID> {
    type ID = ID;
}