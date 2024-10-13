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
        self.db.put(&mut self.wtxn, &workflow.id.to_string(), &serialized)?;
        Ok(())
    }

    fn get_workflow(&self, name: &str) -> anyhow::Result<Option<Workflow>> {
        let db_result = self.db.get(&self.wtxn, name)?;
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