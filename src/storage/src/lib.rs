mod heed;
pub mod records;

use ::heed::{Env, RwTxn};
use crate::heed::Heed;
use crate::records::{Workflow};

pub trait Storage {
    fn create_workflow(&mut self, workflow: Workflow) -> anyhow::Result<()>;

    fn get_queued_workflow(&self, queue_id: &str, id: &str) -> anyhow::Result<Option<Workflow>>;

    fn get_non_queued_workflow(&self, id: &str) -> anyhow::Result<Option<Workflow>>;
}

pub fn new_heed(env: Env, wtxn: RwTxn) -> anyhow::Result<impl Storage + '_> {
    return Heed::new(env, wtxn)
}
