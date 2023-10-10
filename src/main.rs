use blobgraph::blobstore::BlobStore;
use blobgraph::mapstore::MapStore;
use blobgraph::node::Node;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string};
use std::error::Error;
use std::sync::{Arc, RwLock};

fn sj_reader<T: DeserializeOwned>(a: &String) -> Result<T, Box<dyn Error>> {
    match from_str(a) {
        Ok(s) => Ok(s),
        Err(e) => Err(Box::new(e)),
    }
}

fn sj_writter<T: Serialize>(a: &T) -> Result<String, Box<dyn Error>> {
    match to_string(a) {
        Ok(s) => Ok(s),
        Err(e) => Err(Box::new(e)),
    }
}

fn main() {
    let main_store = Arc::new(RwLock::new(MapStore::new()));

    let id1 = "a".to_string();
    let id2 = "b".to_string();

    let _ = main_store
        .write()
        .unwrap()
        .put(&id1, &"Hi".to_string(), sj_writter);

    {
        let reader = main_store.read().unwrap();
        if let Ok(val) = reader.get(&id1, sj_reader::<String>) {
            println!("{}", val);
        }
    }

    let node = Node::from([("files".to_string(), [id1.clone()].to_vec())]);

    let _ = main_store.write().unwrap().put(&id2, &node, sj_writter);

    {
        let reader = main_store.read().unwrap();
        if let Ok(val) = reader.get(&id2, sj_reader::<Node>) {
            println!("{:?}", val);
        }
    }
}
