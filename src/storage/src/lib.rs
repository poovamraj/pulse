mod heed;
pub mod records;

use std::ops::Deref;
use crate::heed::{Heed, KeyValue};
use crate::records::{Workflow};

pub trait Repository {
    fn create_workflow(&mut self, workflow: Workflow) -> anyhow::Result<()>;

    fn get_queued_workflow(&self, queue_id: &str, id: &str) -> anyhow::Result<Option<Workflow>>;

    fn get_non_queued_workflow(&self, id: &str) -> anyhow::Result<Option<Workflow>>;
}

pub fn create_workflow(kv: &mut impl KeyValue<String, String>, workflow: Workflow) -> anyhow::Result<()> {
    let serialized = serde_json::to_string(&workflow)?;
    let key = if let Some(queue_id) = workflow.queue_id {
        format!("{}.{}", queue_id, workflow.id.to_string())
    } else {
        format!("NQ.{}", workflow.id.to_string())
    };
    kv.put(key, serialized)
}

pub fn get_queued_workflow(kv: impl KeyValue<String, String>, queue_id: &str, id: &str) -> anyhow::Result<Option<Workflow>> {
    let db_result = kv.get(format!("{}.{}", queue_id, id))?;
    return match db_result {
        None => {
            Ok(None)
        }
        Some(result) => {
            serde_json::from_str(&result).map_err(anyhow::Error::from)
        }
    }
}

pub fn get_non_queued_workflow(kv: &impl KeyValue<String, String>, id: &str) -> anyhow::Result<Option<Workflow>> {
    let db_result = kv.get(format!("NQ.{}", id))?;
    return match db_result {
        None => {
            Ok(None)
        }
        Some(result) => {
            serde_json::from_str(&result).map_err(anyhow::Error::from)
        }
    }
}

pub fn new_kv() -> anyhow::Result<impl KeyValue<String, String>> {
    Heed::new()
}