use blobgraph::typestore::{TypeStore, TypeFromBlobStore};
use blobgraph::mapstore::MapStore;
use serde_json::{to_string, from_str};
use std::error::Error;

fn sj_reader(a: &String) -> Result<String, Box<dyn Error>> {
    match from_str(a) {
        Ok(s) => Ok(s),
        Err(e) => Err(Box::new(e)),
    }
}

fn sj_writter(a: &String) -> Result<String, Box<dyn Error>> {
    match to_string(a) {
        Ok(s) => Ok(s),
        Err(e) => Err(Box::new(e)),
    }
}

fn main() {
    let mut store = TypeFromBlobStore::new(MapStore::new(), sj_reader, sj_writter);

    let _ = store.put(&"a".to_string(), &"Hi".to_string());
    if let Ok(val) = store.get(&"a".to_string()) {
        println!("{}", val);
    }
}
