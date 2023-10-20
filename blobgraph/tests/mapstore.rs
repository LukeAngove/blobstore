use blobgraph::blobstore::BlobStore;
use blobgraph::mapstore::MapStore;
use blobgraph::node::Node;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string};
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
fn test_mapstore() {
    type SNode = Node<MapStore<String>>;

    let mut main_store = MapStore::new();

    let id1 = "a".to_string();
    let id2 = "b".to_string();

    let str_in = "Hi".to_string();

    let _ = main_store.put(&id1, &str_in, sj_writter);

    let val = main_store
        .get(&id1, sj_reader::<String>)
        .expect("Couldn't get value.");

    assert_eq!(str_in, val);

    let node = SNode::from([("files".to_string(), [id1.clone()].to_vec())]);

    let _ = main_store.put(&id2, &node, sj_writter);

    let val = main_store
        .get(&id2, sj_reader::<SNode>)
        .expect("Couldn't get value.");

    assert_eq!(node, val);
}
