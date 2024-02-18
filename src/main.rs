extern crate tabby_worker_manager;
extern crate tokio;

use tabby_worker_manager::{Result, TabbyWorkerState};

#[tokio::main]
async fn main() -> Result<()> {
    let zookeeper = tabby_worker_manager::TabbyWorkerZookeeper::default();

    let managers = (
        zookeeper.manager("tabby-chat-cpu").await?,
        zookeeper.manager("tabby-chat-gpu").await?,
        zookeeper.manager("tabby-completion-cpu").await?,
        zookeeper.manager("tabby-completion-gpu").await?,
    );

    let state = (
        managers.0.state().await?,
        managers.1.state().await?,
        managers.2.state().await?,
        managers.3.state().await?,
    );

    match state {
        // chat on cpu, completion on gpu
        (TabbyWorkerState::Up, _, _, TabbyWorkerState::Up) => {
            // Always stop first, to avoid out of memory
            managers.0.stop().await?; // cpu chat
            managers.3.stop().await?; // gpu completion
            managers.1.start().await?; // gpu chat
            managers.2.start().await?; // cpu completion
        }
        // chat on gpu, completion on cpu
        (_, TabbyWorkerState::Up, TabbyWorkerState::Up, _) => {
            managers.1.stop().await?; // gpu chat
            managers.2.stop().await?; // cpu completion
            managers.0.start().await?; // cpu chat
            managers.3.start().await?; // gpu completion
        }
        // Any other state switch completion on gpu
        _ => {
            managers.0.stop().await?; // cpu chat
            managers.1.stop().await?; // gpu chat
            managers.2.stop().await?; // cpu completion
            managers.3.stop().await?; // gpu completion

            managers.0.start().await?; // cpu chat
            managers.3.start().await?; // gpu completion
        }
    }

    Ok(())
}
