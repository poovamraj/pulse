use std::fmt::format;
use heed::types::Str;
use heed::{Database, byteorder, RwTxn, Env, Error};
use heed::types::*;
use serde::Serialize;
use crate::records::Workflow;
use crate::Storage;

pub struct Heed<'a> {
    wtxn: RwTxn<'a>,
    db: Database<Str, Str>
}

impl Heed<'_> {
    pub fn new(env: Env, mut wtxn: RwTxn) -> anyhow::Result<impl Storage + '_> {
        let db: Database<Str, Str> = env.create_database(&mut wtxn, None)?;

        return Ok(Heed { wtxn, db })
    }
}

impl Storage for Heed<'_> {
    fn create_workflow(&mut self, workflow: Workflow) -> anyhow::Result<()> {
        let serialized = serde_json::to_string(&workflow)?;
        let key = if let Some(queue_id) = workflow.queue_id {
            format!("{}.{}", queue_id, workflow.id.to_string())
        } else {
            format!("NQ.{}", workflow.id.to_string())
        };
        self.db.put(&mut self.wtxn, &key, &serialized)?;
        Ok(())
    }

    fn get_queued_workflow(&self, queue_id: &str, id: &str) -> anyhow::Result<Option<Workflow>> {
        let db_result = self.db.get(&self.wtxn, &format!("{}.{}", queue_id, id))?;
        return match db_result {
            None => {
                Ok(None)
            }
            Some(result) => {
                serde_json::from_str(result).map_err(anyhow::Error::from)
            }
        }
    }

    fn get_non_queued_workflow(&self, id: &str) -> anyhow::Result<Option<Workflow>> {
        let db_result = self.db.get(&self.wtxn, &format!("NQ.{}", id))?;
        return match db_result {
            None => {
                Ok(None)
            }
            Some(result) => {
                serde_json::from_str(result).map_err(anyhow::Error::from)
            }
        }
    }
}