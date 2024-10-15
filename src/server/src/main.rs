use std::collections::HashMap;

use ulid::Ulid;

use storage::{create_workflow, get_non_queued_workflow, new_kv, Repository};
use storage::records::*;
use storage::records::Workflow;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut storage = new_kv()?;
    let ulid = Ulid::new();

    create_workflow(&mut storage, Workflow {
        name: "".to_string(),
        id: ulid,
        status: WorkflowStatus::Completed,
        queue_id: None,
        state: HashMap::new(),
        error: HashMap::new(),
    }).expect("Cannot Write");

    let result = get_non_queued_workflow(&storage, &ulid.to_string()).expect("Cannot Read");
    println!("{:?}", result);
    let not_found = get_non_queued_workflow(&storage, "asdasdasda").expect("failed on no data");
    println!("{:?}", not_found);
    Ok(())
}