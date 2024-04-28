use std::fmt::Debug;

use serde::{Deserialize, Serialize};

/// Test object that mocks calling different persistences
#[derive(Debug, Serialize, Deserialize)]
struct TestObject {}

/// Database Persist Traits per Domain
trait TestPersist: Send + Sync {
    fn create_test(&self, test: &TestObject) -> Result<bool, String>;
    fn read_test_by_id(&self, id: &str) -> Result<bool, String>;
    fn update_test(&self, test: &TestObject) -> Result<bool, String>;
    fn delete_test_by_id(&self, id: &str) -> Result<bool, String>;
}

/// A persistence that the test object mocks storing
#[derive(Debug, Serialize, Deserialize)]
struct TestStorage {}

impl TestPersist for TestStorage {
    fn create_test(&self, test: &TestObject) -> Result<bool, String> {
        println!("Calling test create for: {:?} from TestStorage.", test);
        Ok(true)
    }

    fn read_test_by_id(&self, id: &str) -> Result<bool, String> {
        println!("Calling test read with id: {id} from TestStorage.");
        Ok(true)
    }

    fn update_test(&self, test: &TestObject) -> Result<bool, String> {
        println!("Calling test update for: {:?} from TestStorage.", test);
        Ok(true)
    }

    fn delete_test_by_id(&self, id: &str) -> Result<bool, String> {
        println!("Calling test delete with id: {id} from TestStorage.");
        Ok(true)
    }
}

/// Another persistence that the test object mocks storing
#[derive(Debug, Serialize, Deserialize)]
struct OtherTestStorage {}

impl TestPersist for OtherTestStorage {
    fn create_test(&self, test: &TestObject) -> Result<bool, String> {
        println!("Calling test create for: {:?} from OtherTestStorage.", test);
        Ok(true)
    }

    fn read_test_by_id(&self, id: &str) -> Result<bool, String> {
        println!("Calling test read with id: {id} from OtherTestStorage.");
        Ok(true)
    }

    fn update_test(&self, test: &TestObject) -> Result<bool, String> {
        println!("Calling test update for: {:?} from OtherTestStorage.", test);
        Ok(true)
    }

    fn delete_test_by_id(&self, id: &str) -> Result<bool, String> {
        println!("Calling test delete with id: {id} from OtherTestStorage.");
        Ok(true)
    }
}

#[test]
/// Instantiates an object and calls on different persistences
fn test_persistence() {
    println!("\nCreating TestObject");
    let my_object = TestObject {};

    /// Calls to the Persist Traits
    impl TestObject {
        pub fn create(&self, client: &dyn TestPersist) -> Result<bool, String> {
            client.create_test(self)
        }

        pub fn read(&self, client: &dyn TestPersist, id: &str) -> Result<bool, String> {
            client.read_test_by_id(id)
        }

        pub fn update(&self, client: &dyn TestPersist) -> Result<bool, String> {
            client.update_test(self)
        }

        pub fn delete(&self, client: &dyn TestPersist, id: &str) -> Result<bool, String> {
            client.delete_test_by_id(id)
        }
    }

    println!("\nTesting TestStorage");
    let test_storage = TestStorage {};
    let create = my_object.create(&test_storage).unwrap();
    assert!(create);
    let read = my_object.read(&test_storage, "Some Unique Key").unwrap();
    assert!(read);
    let update = my_object.update(&test_storage).unwrap();
    assert!(update);
    let delete = my_object.delete(&test_storage, "Some Unique Key").unwrap();
    assert!(delete);

    println!("\nTesting OtherTestStorage");
    let other_test_storage = OtherTestStorage {};
    let create = my_object.create(&other_test_storage).unwrap();
    assert!(create);
    let read = my_object
        .read(&other_test_storage, "Some Unique Key")
        .unwrap();
    assert!(read);
    let update = my_object.update(&other_test_storage).unwrap();
    assert!(update);
    let delete = my_object
        .delete(&other_test_storage, "Some Unique Key")
        .unwrap();
    assert!(delete);
}
