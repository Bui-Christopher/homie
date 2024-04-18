use std::error::Error;
use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::adapter::repository::CRUDOperations;

/// Test object that mocks calling different persistences
#[derive(Debug, Serialize, Deserialize)]
struct TestObject {}

/// A persistence that the test object mocks storing
#[derive(Debug, Serialize, Deserialize)]
struct TestStorage {}

impl<T: Serialize + DeserializeOwned + Debug> CRUDOperations<T> for TestStorage {
    fn create(&self, obj: &T) -> Result<bool, Box<dyn Error>> {
        println!("Test: Create object: {:?}", obj);
        Ok(true)
    }

    fn read(&self, key: &str) -> Result<bool, Box<dyn Error>> {
        println!("Test: Read object by key: {:?}", key);
        Ok(true)
    }

    fn update(&self, obj: &T) -> Result<bool, Box<dyn Error>> {
        println!("Test: Update object: {:?}", obj);
        Ok(true)
    }

    fn delete(&self, key: &str) -> Result<bool, Box<dyn Error>> {
        println!("Test: Delete object by key: {:?}", key);
        Ok(true)
    }
}

/// Another persistence that the test object mocks storing
#[derive(Debug, Serialize, Deserialize)]
struct OtherTestStorage {}

impl<T: Serialize + DeserializeOwned + Debug> CRUDOperations<T> for OtherTestStorage {
    fn create(&self, obj: &T) -> Result<bool, Box<dyn Error>> {
        println!("Other test: Create object: {:?}", obj);
        Ok(true)
    }

    fn read(&self, key: &str) -> Result<bool, Box<dyn Error>> {
        println!("Other test: Read object by key: {:?}", key);
        Ok(true)
    }

    fn update(&self, obj: &T) -> Result<bool, Box<dyn Error>> {
        println!("Other test: Update object: {:?}", obj);
        Ok(true)
    }

    fn delete(&self, key: &str) -> Result<bool, Box<dyn Error>> {
        println!("Other test: Delete object by key: {:?}", key);
        Ok(true)
    }
}

/// Test calls that are dependent by the type of persistence
trait Persistence<T, D: CRUDOperations<T>> {
    fn create(&self, db: &D, obj: &T) -> Result<bool, Box<dyn Error>>;
    fn read(&self, db: &D, key: &str) -> Result<bool, Box<dyn Error>>;
    fn update(&self, db: &D, obj: &T) -> Result<bool, Box<dyn Error>>;
    fn delete(&self, db: &D, key: &str) -> Result<bool, Box<dyn Error>>;
}

impl<D: CRUDOperations<TestObject>> Persistence<TestObject, D> for TestObject {
    fn create(&self, db: &D, obj: &TestObject) -> Result<bool, Box<dyn Error>> {
        db.create(obj)
    }

    fn read(&self, db: &D, key: &str) -> Result<bool, Box<dyn Error>> {
        db.read(key)
    }

    fn update(&self, db: &D, obj: &TestObject) -> Result<bool, Box<dyn Error>> {
        db.update(obj)
    }

    fn delete(&self, db: &D, key: &str) -> Result<bool, Box<dyn Error>> {
        db.delete(key)
    }
}

#[test]
/// Instantiates an object and calls on different persistences
fn test_persistence() {
    let my_object = TestObject {};

    println!("\nTesting TestStorage");
    let test_storage = TestStorage {};
    let create = my_object.create(&test_storage, &my_object).unwrap();
    assert!(create);
    let read = my_object.read(&test_storage, "Read Key").unwrap();
    assert!(read);
    let update = my_object.update(&test_storage, &my_object).unwrap();
    assert!(update);
    let delete = my_object.delete(&test_storage, "Delete Key").unwrap();
    assert!(delete);

    println!("\nTesting OtherTestStorage");
    let other_test_storage = OtherTestStorage {};
    let create = my_object.create(&other_test_storage, &my_object).unwrap();
    assert!(create);
    let read = my_object.read(&other_test_storage, "Read Key").unwrap();
    assert!(read);
    let update = my_object.update(&other_test_storage, &my_object).unwrap();
    assert!(update);
    let delete = my_object.delete(&other_test_storage, "Delete Key").unwrap();
    assert!(delete);
}
