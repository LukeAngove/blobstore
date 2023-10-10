use crate::blobstore::BlobStore;
use digest::Digest;
use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;
use std::marker::PhantomData;

pub type ID = String;

pub trait HashStore<'a> {
    type RawObject: AsRef<[u8]>;
    fn get<T: DeserializeOwned>(
        &'a self,
        id: &ID,
        processor: fn(&'a Self::RawObject) -> Result<T, Box<dyn Error>>,
    ) -> Result<T, Box<dyn Error>>;
    fn put<T: Serialize>(
        &mut self,
        t: &T,
        processor: fn(&T) -> Result<Self::RawObject, Box<dyn Error>>,
    ) -> Result<ID, Box<dyn Error>>;
    fn get_object(&'a self, id: &ID) -> Result<&'a Self::RawObject, Box<dyn Error>>;
    fn put_object(&mut self, obj: &Self::RawObject) -> Result<ID, Box<dyn Error>>;
}

pub struct HashBlobStore<'a, BS: BlobStore<'a>, Hasher: Digest> {
    store: BS,
    phantom: PhantomData<&'a Hasher>,
}

impl<'a, BS: BlobStore<'a>, Hasher: Digest> HashBlobStore<'a, BS, Hasher> {
    pub fn new(store: BS) -> Self {
        Self {
            store,
            phantom: PhantomData,
        }
    }
}

impl<'a, BS: BlobStore<'a>, Hasher: Digest> HashStore<'a> for HashBlobStore<'a, BS, Hasher>
where
    BS::RawObject: AsRef<[u8]>,
{
    type RawObject = BS::RawObject;

    fn get<T: DeserializeOwned>(
        &'a self,
        id: &ID,
        processor: fn(&'a Self::RawObject) -> Result<T, Box<dyn Error>>,
    ) -> Result<T, Box<dyn Error>> {
        self.store.get(id, processor)
    }

    fn put<T: Serialize>(
        &mut self,
        t: &T,
        processor: fn(&T) -> Result<Self::RawObject, Box<dyn Error>>,
    ) -> Result<ID, Box<dyn Error>> {
        let obj = processor(t)?;
        self.put_object(&obj)
    }

    fn get_object(&'a self, id: &ID) -> Result<&'a Self::RawObject, Box<dyn Error>> {
        self.store.get_object(id)
    }

    fn put_object(&mut self, obj: &Self::RawObject) -> Result<ID, Box<dyn Error>> {
        let raw_id = Hasher::digest(obj);
        let id = format!("{:?}", raw_id);
        self.store.put_object(&id, obj)?;
        Ok(id)
    }
}
