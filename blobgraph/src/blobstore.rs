use crate::node::HasID;
use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;

pub type BSResult<T> = Result<T, Box<dyn Error>>;

pub trait BlobStore: HasID {
    type RawObject;

    fn get<T: DeserializeOwned>(
        &self,
        id: &Self::ID,
        processor: fn(Self::RawObject) -> BSResult<T>,
    ) -> Result<T, Box<dyn Error>>;

    fn put<T: Serialize>(
        &mut self,
        id: &Self::ID,
        t: &T,
        processor: fn(&T) -> BSResult<Self::RawObject>,
    ) -> Result<(), Box<dyn Error>>;

    fn get_object(&self, id: &Self::ID) -> Result<Self::RawObject, Box<dyn Error>>;

    fn put_object(&mut self, id: &Self::ID, obj: Self::RawObject) -> Result<(), Box<dyn Error>>;
}
