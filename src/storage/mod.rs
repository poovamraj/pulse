use std::fmt::Error;
use ::heed::{Env, RwTxn};
use crate::storage::heed::Heed;
mod heed;

pub trait Storage {
    async fn create_workflow(&mut self, name: &str) -> anyhow::Result<()>;

    fn get_workflow(&self, name: &str) -> anyhow::Result<Option<u32>>;
}

pub fn new_heed(env: Env, wtxn: &mut RwTxn) -> anyhow::Result<impl Storage> {
    return Heed::new(env, wtxn)
}
