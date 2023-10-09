use std::collections::HashMap;
use std::error::Error;
use serde::{Serialize, Deserialize};
use crate::blobstore::{BlobStore, ID};

pub type Node = HashMap<String, Vec<ID>>;

pub trait TypeStore<'a, T: Serialize+Deserialize<'a>> {
    fn get(&self, id: &ID) -> Result<T, Box<dyn Error>>;
    fn put(&mut self, id: &ID, item: &T) -> Result<(), Box<dyn Error>>;
}

pub struct TypeFromBlobStore<BS: BlobStore, T> {
    bstore: BS,
    reader: fn(&BS::RawObject) -> Result<T, Box<dyn Error>>,
    putter: for<'a> fn(&'a T) -> Result<BS::RawObject, Box<dyn Error>>,
}

impl<BS: BlobStore, T> TypeFromBlobStore<BS, T> {
    pub fn new(bstore: BS, reader: fn(&BS::RawObject) -> Result<T, Box<dyn Error>>, putter: for<'a> fn(&'a T) -> Result<BS::RawObject, Box<dyn Error>>) -> Self {
        Self {
            bstore,
            reader,
            putter,
        }
    }
}

impl<'a, BS: BlobStore, T: Serialize+Deserialize<'a>> TypeStore<'a, T> for TypeFromBlobStore<BS, T> {
    fn get(&self, id: &ID) -> Result<T, Box<dyn Error>> {
        self.bstore.get_object(id, self.reader)
    }

    fn put(&mut self, id: &ID, item: &T) -> Result<(), Box<dyn Error>> {
        let data = (self.putter)(&item).unwrap();
        self.bstore.put_object(id, &data)
    }
}

