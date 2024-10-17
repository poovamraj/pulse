use std::ops::Deref;

use crate::heed::{Heed, KeyValue};
use crate::records::Workflow;

mod heed;
pub mod records;

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

pub fn get_queued_workflow(kv: &impl KeyValue<String, String>, queue_id: &str, id: &str) -> anyhow::Result<Option<Workflow>> {
    let db_result = kv.get(format!("{}.{}", queue_id, id))?;
    return match db_result {
        None => {
            Ok(None)
        }
        Some(result) => {
            serde_json::from_str(&result).map_err(anyhow::Error::from)
        }
    };
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
    };
}

pub fn new_kv() -> anyhow::Result<impl KeyValue<String, String>> {
    Heed::new()
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use ulid::Ulid;

    use crate::{create_workflow, get_non_queued_workflow, get_queued_workflow};
    use crate::heed::MockKeyValue;
    use crate::records::{Workflow, WorkflowStatus};
    use crate::records::WorkflowStatus::Completed;

    #[test]
    fn test_get_non_queued_workflow() {
        let mut mock = MockKeyValue::new();
        mock.expect_get().returning(|key: String| {
            assert_eq!(key, format!("NQ.{}", "item"));
            anyhow::Ok(Some("{\"name\":\"\",\"id\":\"01JAE8D8XP0NDJWWVR6DQY1XMM\",\"status\":{\"in\":\"Completed\"},\"queue_id\":null,\"state\":{},\"error\":{}}".to_string()))
        });
        let result = get_non_queued_workflow(&mock, "item").expect("TODO: panic message");
        let expected: Option<Workflow> = Some(Workflow { name: "".to_string(), id: Ulid::from(2090478060269053672559017708549633684), status: Completed, queue_id: None, state: HashMap::new(), error: HashMap::new() });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_queued_workflow() {
        let mut mock: MockKeyValue<String, String> = MockKeyValue::new();
        mock.expect_get().returning(|key: String| {
            assert_eq!(key, "Q.V");
            anyhow::Ok(None)
        });
        let result = get_queued_workflow(&mock, "Q", "V").expect("TODO: panic message");
        let expected: Option<Workflow> = None;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_write_queued_workflow() {
        let mut mock: MockKeyValue<String, String> = MockKeyValue::new();
        mock.expect_put().returning(|key: String, value: String| {
            assert_eq!(key, "01JAE8D8XP0NDJWWVR6DQY1XMK.01JAE8D8XP0NDJWWVR6DQY1XMM");
            anyhow::Ok(())
        });
        create_workflow(&mut mock, Workflow {
            name: "name".to_string(),
            id: Ulid::from(2090478060269053672559017708549633684),
            status: WorkflowStatus::Pending,
            queue_id: Some(Ulid::from(2090478060269053672559017708549633683)),
            state: Default::default(),
            error: Default::default(),
        }).expect("TODO: panic message");
    }

    #[test]
    fn test_write_non_queued_workflow() {
        let mut mock: MockKeyValue<String, String> = MockKeyValue::new();
        mock.expect_put().returning(|key: String, value: String| {
            assert_eq!(key, "NQ.01JAE8D8XP0NDJWWVR6DQY1XMM");
            anyhow::Ok(())
        });
        create_workflow(&mut mock, Workflow {
            name: "name".to_string(),
            id: Ulid::from(2090478060269053672559017708549633684),
            status: WorkflowStatus::Pending,
            queue_id: None,
            state: Default::default(),
            error: Default::default(),
        }).expect("TODO: panic message");
    }
}