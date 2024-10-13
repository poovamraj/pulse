mod heed;

use ::heed::{Env, RwTxn};
use crate::heed::Heed;

pub trait Storage {
    fn create_workflow(&mut self, name: &str) -> anyhow::Result<()>;

    fn get_workflow(&self, name: &str) -> anyhow::Result<Option<u32>>;
}

pub fn new_heed(env: Env, wtxn: RwTxn) -> anyhow::Result<impl Storage + '_> {
    return Heed::new(env, wtxn)
}
