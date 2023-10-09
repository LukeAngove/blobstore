use std::error::Error;

pub type ID = String;

pub trait BlobStore {
    type RawObject;
    fn get_object<T>(&self, id: &ID, processor: fn(&Self::RawObject) -> Result<T, Box<dyn Error>>) -> Result<T, Box<dyn Error>>;
    fn put_object(&mut self, id: &ID, obj: &Self::RawObject) -> Result<(), Box<dyn Error>>;
}


