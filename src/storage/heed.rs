use std::fs;
use std::path::Path;
use heed::types::Str;
use heed::{EnvOpenOptions, Database, byteorder, RwTxn, Env};
use heed::types::*;
use crate::storage::Storage;

pub struct Heed {
    env: Env,
    db: Database<Str, U32<byteorder::NativeEndian>>
}

impl Heed {
    pub fn new() -> anyhow::Result<Self> {
        let path = Path::new("target").join("heed.mdb");
        fs::create_dir_all(&path)?;

        let env = unsafe { EnvOpenOptions::new().open(&path) }?;

        // We open the default unnamed database
        let mut wtxn = env.write_txn()?;
        let db: Database<Str, U32<byteorder::NativeEndian>> = env.create_database(&mut wtxn, None)?;

        return Ok(Heed { env: env.clone(), db })
    }
}

impl Storage for Heed {
    async fn create_workflow(&self, name: &str) -> anyhow::Result<()> {
        let mut wtxn = self.env.write_txn()?;
        self.db.put(&mut wtxn, name, &7)?;
        Ok(())
    }

    fn get_workflow(&self, name: &str) -> anyhow::Result<Option<u32>> {
        let mut rtxn = self.env.read_txn()?;
        let result = self.db.get(&mut rtxn, name)?;
        Ok(result)
    }
}