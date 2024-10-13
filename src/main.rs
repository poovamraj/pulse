use std::fs;
use std::path::Path;
use heed::EnvOpenOptions;
use crate::storage::Storage;

mod storage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("target").join("heed.mdb");
    fs::create_dir_all(&path)?;
    let env = unsafe { EnvOpenOptions::new().open(&path) }?;
    // We open the default unnamed database
    let mut wtxn = env.write_txn()?;


    let mut heed = storage::new_heed(env, &mut wtxn).expect("Cannot create");
    heed.create_workflow("test").await.expect("Cannot Write");
    let result = heed.get_workflow("test").expect("Cannot Read");
    println!("{:?}", result);
    Ok(())
}

