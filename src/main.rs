use crate::storage::Storage;

mod storage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let heed = storage::new_heed().expect("Cannot create");
    heed.create_workflow("test").await.expect("Cannot Write");
    let result = heed.get_workflow("test").expect("Cannot Read");
    println!("{:?}", result);
    Ok(())
}

