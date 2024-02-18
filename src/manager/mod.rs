mod docker;

use async_trait::async_trait;

use crate::worker::TabbyWorkerConfig;
use crate::worker::TabbyWorkerState;
use crate::Result;
use crate::TabbyWorkerHandler;

#[async_trait]
pub trait Manager {
    async fn start(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
    async fn state(&self) -> Result<TabbyWorkerState>;
}

#[derive(Default)]
pub struct ManagerFactory {}

impl ManagerFactory {
    pub fn create_manager(&self, config: TabbyWorkerConfig) -> Box<dyn Manager> {
        match config.handler {
            TabbyWorkerHandler::Docker(container_name) => {
                Box::new(docker::DockerManager::new(container_name))
            }
        }
    }
}
