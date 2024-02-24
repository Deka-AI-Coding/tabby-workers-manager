use tabby_worker_manager::{Error, Result, TabbyWorkerState, TabbyWorkerZookeeper};

use serde::Serialize;

pub mod prelude {
    pub use super::WorkerInfo;
}

#[derive(Serialize)]
pub struct WorkerInfo {
    pub name: String,
    pub state: TabbyWorkerState,
}

pub struct Service {
    zookeeper: TabbyWorkerZookeeper,
}

impl Service {
    pub fn new(zookeeper: TabbyWorkerZookeeper) -> Self {
        Self { zookeeper }
    }

    /// Gets all workers info
    pub async fn get_workers(&self) -> Result<Vec<WorkerInfo>> {
        let names = self.zookeeper.manager_names().await?;

        let mut infos = vec![];
        for name in names {
            let manager = self.zookeeper.manager(&name).await?;
            let state = manager.state().await?;
            let info = WorkerInfo { name, state };
            infos.push(info);
        }

        Ok(infos)
    }

    /// Gets worker info for given name
    pub async fn get_worker(&self, name: &str) -> Result<WorkerInfo> {
        let name = self
            .zookeeper
            .manager_names()
            .await?
            .into_iter()
            .find(|n| n == name)
            .ok_or(Error::WorkerNotFound(name.to_owned()))?;

        let manager = self.zookeeper.manager(&name).await?;
        let state = manager.state().await?;
        let info = WorkerInfo { name, state };

        Ok(info)
    }

    /// Starts worker with given name
    pub async fn start_worker(&self, name: &str) -> Result<WorkerInfo> {
        let name = self
            .zookeeper
            .manager_names()
            .await?
            .into_iter()
            .find(|n| n == name)
            .ok_or(Error::WorkerNotFound(name.to_owned()))?;

        let manager = self.zookeeper.manager(&name).await?;
        manager.start().await?;

        self.get_worker(&name).await
    }

    /// Starts worker with given name
    pub async fn stop_worker(&self, name: &str) -> Result<WorkerInfo> {
        let name = self
            .zookeeper
            .manager_names()
            .await?
            .into_iter()
            .find(|n| n == name)
            .ok_or(Error::WorkerNotFound(name.to_owned()))?;

        let manager = self.zookeeper.manager(&name).await?;
        manager.stop().await?;

        self.get_worker(&name).await
    }
}
