
use std::path::Path;

use std::io::Cursor;
use leveldb::database::Database;
use leveldb::iterator::Iterable;
use leveldb::options::{Options,WriteOptions,ReadOptions};
use leveldb::database::kv::KV;
use bitcoin::network::message_blockdata::Inventory;
use bitcoin::network::serialize::{RawEncoder, BitcoinHash};
use bitcoin::network::serialize::serialize;

pub fn test_ldb() {

  let mut options = Options::new();
  options.create_if_missing = true;
  let mut database = match Database::open(Path::new("./data"), options) {
      Ok(db) => { db },
      Err(e) => { panic!("failed to open database: {:?}", e) }
  };

  let write_opts = WriteOptions::new();
  match database.put(write_opts, 1, ("hello".to_string().as_bytes())) {
      Ok(_) => { () },
      Err(e) => { panic!("failed to write to database: {:?}", e) }
  };

  let read_opts = ReadOptions::new();
  let res = database.get(read_opts,  1);
  match res {
    Ok(data) => {
        println!("{:?}", data);
    //   assert!(data.is_some());
    //   assert_eq!(data, Some(vec![1]));
    }
    Err(e) => { panic!("failed reading data: {:?}", e) }
  }

  let read_opts = ReadOptions::new();
  let mut iter = database.iter(read_opts);
  let entry = iter.next();
}

pub fn save_blocks(inv: Vec<Inventory>) {

  let mut options = Options::new();
  options.create_if_missing = true;
  let mut database = match Database::open(Path::new("./data"), options) {
      Ok(db) => { db },
      Err(e) => { panic!("failed to open database: {:?}", e) }
  };

  let write_opts = WriteOptions::new();

  for block in inv {

    match serialize(&block) {
        Ok(data) => {
            //println!("data {:?}", &data[..]);
            match database.put(write_opts, 1, &data[..] ) {
                Ok(_) => { () },
                Err(e) => { panic!("failed to write to database: {:?}", e) }
            };
        },
        Err(e) => { panic!("failed to serialize") }


    }
  }
}

pub fn read_blocks() {

      let mut options = Options::new();
      options.create_if_missing = true;
      let mut database = match Database::open(Path::new("./data"), options) {
          Ok(db) => { db },
          Err(e) => { panic!("failed to open database: {:?}", e) }
      };

  let read_opts = ReadOptions::new();
  let res = database.get(read_opts,  1);
  match res {
    Ok(data) => {
        println!("{:?}", data);
    //   assert!(data.is_some());
    //   assert_eq!(data, Some(vec![1]));
    }
    Err(e) => { panic!("failed reading data: {:?}", e) }
  }

  let read_opts = ReadOptions::new();
  let mut iter = database.iter(read_opts);
  let entry = iter.next();
}
