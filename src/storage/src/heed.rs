use std::fs;
use std::path::Path;
use heed::types::Str;
use heed::{Database, EnvOpenOptions, Env};

pub struct Heed {
    db: Database<Str, Str>,
    env: Env
}

pub trait KeyValue<K,V> {
    fn put(&mut self, key: K, value: V) -> anyhow::Result<()>;

    fn get(&self, key: K) -> anyhow::Result<Option<V>>;
}

impl KeyValue<String, String> for Heed {
    fn put(&mut self, key: String, value: String) -> anyhow::Result<()> {
        let mut wtxn = self.env.write_txn()?;
        self.db.put(&mut wtxn, &key, &value)?;
        wtxn.commit()?;
        Ok(())
    }

    fn get(&self, key: String) -> anyhow::Result<Option<String>> {
        let rtxn = self.env.read_txn()?;
        let Some(result) = self.db.get(&rtxn, &key)? else { return anyhow::Ok(None) };
        anyhow::Ok(Some(String::from(result)))
    }
}

impl Heed {
    pub fn new() -> anyhow::Result<Heed> {
        let path = Path::new("target").join("heed.mdb");
        fs::create_dir_all(&path).expect("failure creating dir");
        let env = unsafe { EnvOpenOptions::new().map_size(10 * 1024 * 1024)
            .max_dbs(3000).open(&path) }.expect("failure with env");

        let mut wtxn = env.write_txn().expect("failure with write");
        let db = env.create_database(&mut wtxn, None)?;
        wtxn.commit()?;
        return Ok(Heed { db, env })
    }
}