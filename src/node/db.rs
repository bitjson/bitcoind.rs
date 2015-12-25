use std::path::Path;

use lmdb::{Cursor, Environment, Transaction, WriteFlags, DatabaseFlags};

use bitcoin::network::message_blockdata::Inventory;
use bitcoin::network::serialize::serialize;

pub struct DataStore {
    environment: Environment,
}

impl DataStore {
    pub fn new(path: &Path) -> DataStore {

        match Environment::new().set_max_dbs(2).open(path) {
            Ok(env) => DataStore { environment: env },
            Err(e) => panic!("Unable to open environment {:?}", e),
        }
    }

    pub fn save_blocks(&self, inv: Vec<Inventory>) {


        let db_name = "blocks".to_string();
        let db = self.environment
                     .create_db(Some(&db_name), DatabaseFlags::empty())
                     .expect("Unable to open database");

        let mut txn = self.environment.begin_rw_txn().unwrap();

        for block in inv {

            match serialize(&block) {
                Ok(data) => {
                    println!("data {:?}", &data[..]);

                    // let bhash = block.hash;
                    match txn.put(db, &[1u8], &data, WriteFlags::empty()) {
                        Ok(_) => (),
                        Err(e) => panic!("failed to write to database: {:?}", e),
                    };
                }
                Err(e) => println!("failed {:?}", e),

            }
        }

        txn.commit().unwrap();
    }

    pub fn read_blocks(&self) {

        let db_name = "blocks".to_string();
        let db = self.environment
                     .create_db(Some(&db_name), DatabaseFlags::empty())
                     .expect("Unable to open database");

        let txn = self.environment.begin_ro_txn().expect("asd");

        match txn.get(db, &[1u8]) {
            Ok(data) => {

                println!("success {:?}", data);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }


    }
}
