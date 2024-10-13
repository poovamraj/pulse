use std::fs;
use std::path::Path;
use heed::EnvOpenOptions;
use storage::new_heed;
use storage::Storage;

fn main() {
    let path = Path::new("target").join("heed.mdb");
    fs::create_dir_all(&path).expect("failure creating dir");
    let env = unsafe { EnvOpenOptions::new().open(&path) }.expect("failure with env");

    let mut wtxn = env.write_txn().expect("failure with write"); // We open the default unnamed database
    let mut heed = new_heed(env.clone(), wtxn).expect("Cannot create");

    heed.create_workflow("test").expect("Cannot Write");
    let result = heed.get_workflow("test").expect("Cannot Read");
    println!("{:?}", result);
}

