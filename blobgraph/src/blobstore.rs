use crate::node::HasID;
use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;

pub trait BlobStore: HasID {
    type RawObject;

    fn get<T: DeserializeOwned>(
        &self,
        id: &Self::ID,
        processor: fn(Self::RawObject) -> Result<T, Box<dyn Error>>,
    ) -> Result<T, Box<dyn Error>>;
    fn put<T: Serialize>(
        &mut self,
        id: &Self::ID,
        t: &T,
        processor: fn(&T) -> Result<Self::RawObject, Box<dyn Error>>,
    ) -> Result<(), Box<dyn Error>>;
    fn get_object(&self, id: &Self::ID) -> Result<Self::RawObject, Box<dyn Error>>;
    fn put_object(&mut self, id: &Self::ID, obj: Self::RawObject) -> Result<(), Box<dyn Error>>;
}
