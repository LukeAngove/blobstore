use blobgraph::hashstore::{HashBlobStore, HashStore};
use blobgraph::mapstore::MapStore;
use blobgraph::node::Node;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string};
use sha2::Sha256;
use std::error::Error;

fn sj_reader<T: DeserializeOwned>(a: String) -> Result<T, Box<dyn Error>> {
    match from_str(&a) {
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

#[test]
fn test_hash_store() {
    type Store = HashBlobStore<MapStore<String>, Sha256>;
    type SNode = Node<Store>;

    let mut main_store = Store::new(MapStore::new());

    let str_in = "Hi".to_string();

    let id1 = main_store
        .put(&str_in, sj_writter)
        .expect("Failed to put object");
    let val = main_store
        .get(&id1, sj_reader::<String>)
        .expect("Failed to get result");
    assert_eq!(str_in, val);

    let node = SNode::from([("files".to_string(), [id1.clone()].to_vec())]);
    let id2 = main_store
        .put(&node, sj_writter)
        .expect("Failed to put object");
    let val = main_store
        .get(&id2, sj_reader::<SNode>)
        .expect("Failed to get result");
    assert_eq!(node, val);

    let val = main_store
        .get(&id1, sj_reader::<String>)
        .expect("Failed to get result");
    assert_eq!(str_in, val);
}
