use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use super::common::FilePersist;

/// Test object that mocks calling different persistences
#[derive(Debug, Serialize, Deserialize)]
struct TestObject {}

/// A persistence that the test object mocks storing
#[derive(Debug, Serialize, Deserialize)]
struct TestStorage {}

/// Another persistence that the test object mocks storing
#[derive(Debug, Serialize, Deserialize)]
struct OtherTestStorage {}

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
