use std::collections::HashMap;

pub trait HasID {
    type ID;
}

pub type Node<T> = HashMap<String, Vec<<T as HasID>::ID>>;
