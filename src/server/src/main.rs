use std::collections::HashMap;
use ulid::Ulid;
use storage::new_heed;
use storage::records::Workflow;
use storage::Storage;
use storage::records::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut storage = new_heed()?;

    let ulid = Ulid::new();
    storage.create_workflow(Workflow {
        name: "".to_string(),
        id: ulid,
        status: WorkflowStatus::Completed,
        queue_id: None,
        state: HashMap::new(),
        error: HashMap::new(),
    }).expect("Cannot Write");
    let result = storage.get_non_queued_workflow(&ulid.to_string()).expect("Cannot Read");
    println!("{:?}", result);
    Ok(())
}