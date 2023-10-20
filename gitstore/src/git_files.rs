use blobgraph::blobstore::BlobStore;
use blobgraph::node::HasID;
use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

pub struct GitStore {
    path: String,
}

impl GitStore {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    fn id_to_path(&self, id: &str) -> Result<String, Box<dyn Error>> {
        Ok(format!(
            "{}/{}/{}",
            self.path,
            id.get(0..2).ok_or("failed")?,
            id.get(2..).ok_or("failed")?
        ))
    }
}

impl Default for GitStore {
    fn default() -> Self {
        Self {
            path: ".git/objects".into(),
        }
    }
}

impl HasID for GitStore {
    type ID = String;
}

impl BlobStore for GitStore {
    type RawObject = BufReader<File>;

    fn get<T: DeserializeOwned>(
        &self,
        id: &Self::ID,
        process: fn(Self::RawObject) -> Result<T, Box<dyn Error>>,
    ) -> Result<T, Box<dyn Error>> {
        if let Ok(d) = self.get_object(id) {
            process(d)
        } else {
            Err(Box::<dyn Error>::from("No such key"))
        }
    }

    fn put<T: Serialize>(
        &mut self,
        id: &Self::ID,
        t: &T,
        process: fn(&T) -> Result<Self::RawObject, Box<dyn Error>>,
    ) -> Result<(), Box<dyn Error>> {
        let data = process(t)?;
        self.put_object(id, data)
    }

    fn get_object(&self, id: &Self::ID) -> Result<Self::RawObject, Box<dyn Error>> {
        let path = self.id_to_path(id)?;
        let file = File::open(path)?;
        Ok(BufReader::new(file))
    }

    fn put_object(&mut self, _id: &Self::ID, _obj: Self::RawObject) -> Result<(), Box<dyn Error>> {
        Err(Box::<dyn Error>::from("Put not supported."))
    }
}
