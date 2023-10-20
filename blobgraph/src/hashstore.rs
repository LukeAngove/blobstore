use crate::blobstore::BlobStore;
use crate::node::HasID;
use digest::{Digest, Output};
use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;
use std::marker::PhantomData;

trait TyEq {}

impl<T> TyEq for (T, T) {}

pub trait HashStore: HasID {
    type HashOutput;
    type RawObject: AsRef<[u8]>;

    fn get<T: DeserializeOwned>(
        &self,
        id: &<Self as HasID>::ID,
        processor: fn(Self::RawObject) -> Result<T, Box<dyn Error>>,
    ) -> Result<T, Box<dyn Error>>;

    fn put<T: Serialize>(
        &mut self,
        t: &T,
        processor: fn(&T) -> Result<Self::RawObject, Box<dyn Error>>,
    ) -> Result<<Self as HasID>::ID, Box<dyn Error>>;

    fn get_object(
        &self,
        id: &<Self as HasID>::ID,
    ) -> Result<Self::RawObject, Box<dyn Error>>;
    fn put_object(&mut self, obj: Self::RawObject) -> Result<<Self as HasID>::ID, Box<dyn Error>>;
    fn digest_into_id(raw_id: Self::HashOutput) -> <Self as HasID>::ID;
}

pub struct HashBlobStore<BS: BlobStore<ID = String>, Hasher: Digest> {
    store: BS,
    phantom: PhantomData<Hasher>,
}

impl<BS: BlobStore<ID = String>, Hasher: Digest> HashBlobStore<BS, Hasher> {
    pub fn new(store: BS) -> Self {
        Self {
            store,
            phantom: PhantomData,
        }
    }
}

impl<BS: BlobStore<ID = String>, Hasher: Digest> HasID for HashBlobStore<BS, Hasher> {
    type ID = BS::ID;
}

impl<BS: BlobStore<ID = String>, Hasher: Digest> HashStore
    for HashBlobStore<BS, Hasher>
where
    BS::RawObject: AsRef<[u8]>,
{
    type HashOutput = Output<Hasher>;
    type RawObject = BS::RawObject;

    fn get<T: DeserializeOwned>(
        &self,
        id: &Self::ID,
        processor: fn(Self::RawObject) -> Result<T, Box<dyn Error>>,
    ) -> Result<T, Box<dyn Error>> {
        self.store.get(id, processor)
    }

    fn put<T: Serialize>(
        &mut self,
        t: &T,
        processor: fn(&T) -> Result<Self::RawObject, Box<dyn Error>>,
    ) -> Result<Self::ID, Box<dyn Error>> {
        let obj = processor(t)?;
        self.put_object(obj)
    }

    fn get_object(&self, id: &Self::ID) -> Result<Self::RawObject, Box<dyn Error>> {
        self.store.get_object(id)
    }

    fn put_object(&mut self, obj: Self::RawObject) -> Result<Self::ID, Box<dyn Error>> {
        let raw_id = Hasher::digest(&obj);
        let id = Self::digest_into_id(raw_id);
        self.store.put_object(&id, obj)?;
        Ok(id)
    }

    fn digest_into_id(raw_id: Self::HashOutput) -> Self::ID {
        format!("{:?}", raw_id)
    }
}
