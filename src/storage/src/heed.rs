use std::fs;
use std::path::Path;
use heed::types::Str;
use heed::{Database, RwTxn, EnvOpenOptions, Env};
use crate::records::Workflow;
use crate::Storage;

pub struct Heed {
    db: Database<Str, Str>,
    env: Env
}

impl Heed {
    pub fn new() -> anyhow::Result<impl Storage> {
        let path = Path::new("target").join("heed.mdb");
        fs::create_dir_all(&path).expect("failure creating dir");
        let env = unsafe { EnvOpenOptions::new().map_size(10 * 1024 * 1024)
            .max_dbs(3000).open(&path) }.expect("failure with env");

        let mut wtxn = env.write_txn().expect("failure with write");
        let db = env.create_database(&mut wtxn, None)?;
        return Ok(Heed { db, env: env.clone() })
    }
}

impl Storage for Heed {
    fn create_workflow(&mut self, workflow: Workflow) -> anyhow::Result<()> {
        let serialized = serde_json::to_string(&workflow)?;
        let key = if let Some(queue_id) = workflow.queue_id {
            format!("{}.{}", queue_id, workflow.id.to_string())
        } else {
            format!("NQ.{}", workflow.id.to_string())
        };
        let mut wtxn = self.env.write_txn()?;
        self.db.put(&mut wtxn, &key, &serialized)?;
        wtxn.commit()?;
        Ok(())
    }

    fn get_queued_workflow(&self, queue_id: &str, id: &str) -> anyhow::Result<Option<Workflow>> {
        let rtxn = self.env.read_txn()?;
        let db_result = self.db.get(&rtxn, &format!("{}.{}", queue_id, id))?;
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
        let rtxn = self.env.read_txn()?;
        let db_result = self.db.get(&rtxn, &format!("NQ.{}", id))?;
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