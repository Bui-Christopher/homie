use std::error::Error;

use serde::{Deserialize, Serialize};

pub trait CrudOperations<T> {
    fn create(&self, obj: &T) -> Result<bool, Box<dyn Error>>;
    fn read(&self, obj: &T) -> Result<bool, Box<dyn Error>>;
    fn update(&self, obj: &T) -> Result<bool, Box<dyn Error>>;
    fn delete(&self, obj: &T) -> Result<bool, Box<dyn Error>>;
}

// impl<T, D: CrudOperations<T>> CrudOperations<T> for D {
//     fn create(&self, obj: &T) -> Result<bool, Box<dyn Error>> {
//         D::create(self, obj)
//     }
//
//     fn read(&self, obj: &T) -> Result<bool, Box<dyn Error>> {
//         D::read(self, obj)
//     }
//
//     fn update(&self, obj: &T) -> Result<bool, Box<dyn Error>> {
//         D::update(self, obj)
//     }
//
//     fn delete(&self, obj: &T) -> Result<bool, Box<dyn Error>> {
//         D::delete(self, obj)
//     }
// }

pub struct MongoDatabase {}

impl<'a, T: Serialize + Deserialize<'a> + std::fmt::Debug> CrudOperations<T> for MongoDatabase {
    fn create(&self, obj: &T) -> Result<bool, Box<dyn Error>> {
        println!("Mongo: Create object: {:?}", obj);
        Ok(true)
    }

    fn read(&self, obj: &T) -> Result<bool, Box<dyn Error>> {
        println!("Mongo: Read object: {:?}", obj);
        Ok(true)
    }

    fn update(&self, obj: &T) -> Result<bool, Box<dyn Error>> {
        println!("Mongo: Update object: {:?}", obj);
        Ok(true)
    }

    fn delete(&self, obj: &T) -> Result<bool, Box<dyn Error>> {
        println!("Mongo: Delete object: {:?}", obj);
        Ok(true)
    }
}

pub struct FileStorage {}

impl<'a, T: Serialize + Deserialize<'a> + std::fmt::Debug> CrudOperations<T> for FileStorage {
    fn create(&self, obj: &T) -> Result<bool, Box<dyn Error>> {
        println!("File: Create object: {:?}", obj);
        Ok(true)
    }

    fn read(&self, obj: &T) -> Result<bool, Box<dyn Error>> {
        println!("File: Read object: {:?}", obj);
        Ok(true)
    }

    fn update(&self, obj: &T) -> Result<bool, Box<dyn Error>> {
        println!("File: Update object: {:?}", obj);
        Ok(true)
    }

    fn delete(&self, obj: &T) -> Result<bool, Box<dyn Error>> {
        println!("File: Delete object: {:?}", obj);
        Ok(true)
    }
}

pub trait BasicCRUD<T, D: CrudOperations<T>> {
    fn create(&self, db: &D, obj: &T) -> Result<bool, Box<dyn Error>>;
    fn read(&self, db: &D, obj: &T) -> Result<bool, Box<dyn Error>>;
    fn update(&self, db: &D, obj: &T) -> Result<bool, Box<dyn Error>>;
    fn delete(&self, db: &D, obj: &T) -> Result<bool, Box<dyn Error>>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyObject {}

impl<T, D: CrudOperations<T>> BasicCRUD<T, D> for MyObject {
    fn create(&self, db: &D, obj: &T) -> Result<bool, Box<dyn Error>> {
        db.create(obj)
    }

    fn read(&self, db: &D, obj: &T) -> Result<bool, Box<dyn Error>> {
        db.read(obj)
    }

    fn update(&self, db: &D, obj: &T) -> Result<bool, Box<dyn Error>> {
        db.update(obj)
    }

    fn delete(&self, db: &D, obj: &T) -> Result<bool, Box<dyn Error>> {
        db.delete(obj)
    }
}

