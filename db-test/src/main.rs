use crate::db::{BasicCRUD, FileStorage, MongoDatabase, MyObject};

mod db;

fn main() {
    let my_object = MyObject {};
    let mongo_db = MongoDatabase {};

    println!("\nChecking MongoDB");
    let create = my_object.create(&mongo_db, &my_object).unwrap();
    assert!(create);
    let read = my_object.read(&mongo_db, &my_object).unwrap();
    assert!(read);
    let update = my_object.update(&mongo_db, &my_object).unwrap();
    assert!(update);
    let delete = my_object.delete(&mongo_db, &my_object).unwrap();
    assert!(delete);

    println!("\nChecking FileStorage");
    let file_storage = FileStorage {};
    let create = my_object.create(&file_storage, &my_object).unwrap();
    assert!(create);
    let read = my_object.read(&file_storage, &my_object).unwrap();
    assert!(read);
    let update = my_object.update(&file_storage, &my_object).unwrap();
    assert!(update);
    let delete = my_object.delete(&file_storage, &my_object).unwrap();
    assert!(delete);
}

