use std::fs;
use std::path::Path;
use heed::EnvOpenOptions;
use ulid::Ulid;
use storage::new_heed;
use storage::records::Workflow;
use storage::Storage;
use storage::records::*;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("target").join("heed.mdb");
    fs::create_dir_all(&path).expect("failure creating dir");
    let env = unsafe { EnvOpenOptions::new().map_size(10 * 1024 * 1024) // 10MB
            .max_dbs(3000).open(&path) }.expect("failure with env");

    let wtxn = env.write_txn().expect("failure with write"); // We open the default unnamed database
    let mut heed = new_heed(env.clone(), wtxn).expect("Cannot create");

    let ulid = Ulid::new();
    heed.create_workflow(Workflow {
        name: "".to_string(),
        id: ulid,
        status: WorkflowStatus::Completed,
        queue_id: None,
        state: "".to_string(),
        error: "".to_string(),
    }).expect("Cannot Write");
    let result = heed.get_workflow(&ulid.to_string()).expect("Cannot Read");
    println!("{:?}", result);
    Ok(())
}

