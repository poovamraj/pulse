use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Serialize, Deserialize)]
#[serde(tag = "mode")]
enum QueueMode {
    Sequential,
    Parallel { max_concurrent_executions: u64 },
}

#[derive(Serialize, Deserialize)]
pub struct Queue {
    id: Ulid,
    mode: QueueMode,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Workflow {
    pub name: String,
    pub id: Ulid,
    pub status: WorkflowStatus,
    pub queue_id: Option<Ulid>,
    pub state: HashMap<String, String>,
    pub error: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "in")]
pub enum WorkflowStatus {
    Pending,
    Running,
    Completed,
    Failed,
}