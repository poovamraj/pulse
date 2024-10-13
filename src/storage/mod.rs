use ::heed::{Env, RwTxn};
use crate::storage::heed::Heed;
mod heed;

pub trait Storage {
    async fn create_workflow(&mut self, name: &str) -> anyhow::Result<()>;

    fn get_workflow(&self, name: &str) -> anyhow::Result<Option<u32>>;
}

pub fn new_heed<'a>(env: Env, wtxn: &'a mut RwTxn<'a>) -> anyhow::Result<impl Storage + 'a> {
    return Heed::new(env, wtxn)
}
