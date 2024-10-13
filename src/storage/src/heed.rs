use heed::types::Str;
use heed::{Database, byteorder, RwTxn, Env};
use heed::types::*;
use crate::Storage;

pub struct Heed<'a> {
    wtxn: RwTxn<'a>,
    db: Database<Str, U32<byteorder::NativeEndian>>
}

impl Heed<'_> {
    pub fn new(env: Env, mut wtxn: RwTxn) -> anyhow::Result<impl Storage + '_> {
        let db: Database<Str, U32<byteorder::NativeEndian>> = env.create_database(&mut wtxn, None)?;

        return Ok(Heed { wtxn, db })
    }
}

impl Storage for Heed<'_> {
    fn create_workflow(&mut self, name: &str) -> anyhow::Result<()> {
        self.db.put(&mut self.wtxn, name, &7)?;
        Ok(())
    }

    fn get_workflow(&self, name: &str) -> anyhow::Result<Option<u32>> {
        let result = self.db.get(&self.wtxn, name)?;
        Ok(result)
    }
}