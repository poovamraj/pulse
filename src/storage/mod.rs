use std::fmt::Error;
use crate::storage::heed::Heed;
mod heed;

pub trait Storage {
    async fn create_workflow(&self, name: &str) -> anyhow::Result<()>;

    fn get_workflow(&self, name: &str) -> anyhow::Result<Option<u32>>;
}

pub fn new_heed() -> anyhow::Result<impl Storage> {
    return Heed::new()
}
