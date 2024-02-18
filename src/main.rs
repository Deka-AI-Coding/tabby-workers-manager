extern crate tabby_worker_manager;
extern crate tokio;

use std::io::Result;

use tabby_worker_manager::list_images;

#[tokio::main]
async fn main() -> Result<()> {
    list_images().await;
    Ok(())
}
