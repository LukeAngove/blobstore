use std::collections::HashMap;
use crate::blobstore::ID;

pub type Node = HashMap<String, Vec<ID>>;
