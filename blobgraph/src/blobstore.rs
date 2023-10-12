use crate::node::HasID;
use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;

pub trait BlobStore<'a>: HasID {
    type RawObject;

    fn get<T: DeserializeOwned>(
        &'a self,
        id: &Self::ID,
        processor: fn(&'a Self::RawObject) -> Result<T, Box<dyn Error>>,
    ) -> Result<T, Box<dyn Error>>;
    fn put<T: Serialize>(
        &mut self,
        id: &Self::ID,
        t: &T,
        processor: fn(&T) -> Result<Self::RawObject, Box<dyn Error>>,
    ) -> Result<(), Box<dyn Error>>;
    fn get_object(&'a self, id: &Self::ID) -> Result<&'a Self::RawObject, Box<dyn Error>>;
    fn put_object(&mut self, id: &Self::ID, obj: &Self::RawObject) -> Result<(), Box<dyn Error>>;
}
