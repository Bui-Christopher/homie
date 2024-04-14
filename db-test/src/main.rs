use crate::database::file::FileStorage;
use crate::model::myobject::{MyObject, Persistence};

mod database;
mod model;

fn main() {
    let storage = FileStorage {};
    let object = MyObject {};
    let create = object.create(&storage).unwrap();
    assert!(create);
    let read = object.read(&storage, "Read Key").unwrap();
    assert!(read);
    let update = object.update(&storage, &object).unwrap();
    assert!(update);
    let delete = object.delete(&storage, "Read Key").unwrap();
    assert!(delete);
}
