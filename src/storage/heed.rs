use heed::types::Str;
use heed::{Database, byteorder, RwTxn, Env};
use heed::types::*;
use crate::storage::Storage;

pub struct Heed<'a> {
    wtxn: &'a mut RwTxn<'a>,
    db: Database<Str, U32<byteorder::NativeEndian>>
}

impl Heed<'_> {
    pub fn new<'a>(env: Env, wtxn: &'a mut RwTxn<'a>) -> anyhow::Result<&'a mut Heed<'a>> {
        let db: Database<Str, U32<byteorder::NativeEndian>> = env.create_database(wtxn, None)?;

        return Ok(&mut Heed { wtxn, db })
    }
}

impl Storage for &mut Heed<'_> {
    async fn create_workflow(&mut self, name: &str) -> anyhow::Result<()> {
        self.db.put(&mut self.wtxn, name, &7)?;
        Ok(())
    }

    fn get_workflow(&self, name: &str) -> anyhow::Result<Option<u32>> {
        let result = self.db.get(&self.wtxn, name)?;
        Ok(result)
    }
}