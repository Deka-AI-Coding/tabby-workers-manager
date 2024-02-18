use async_trait::async_trait;
use bollard::service::ContainerStateStatusEnum;
use bollard::Docker;

use super::Manager;
use super::Result;
use super::TabbyWorkerState;

pub struct DockerManager {
    container_name: String,
    docker: Docker,
}

impl DockerManager {
    pub fn new(container_name: String) -> Self {
        Self {
            container_name,
            docker: Docker::connect_with_socket_defaults().unwrap(),
        }
    }
}

#[async_trait]
impl Manager for DockerManager {
    async fn start(&self) -> Result<()> {
        Ok(self
            .docker
            .start_container::<String>(&self.container_name, None)
            .await?)
    }

    async fn stop(&self) -> Result<()> {
        Ok(self
            .docker
            .stop_container(&self.container_name, None)
            .await?)
    }

    async fn state(&self) -> Result<TabbyWorkerState> {
        let inspect = self
            .docker
            .inspect_container(&self.container_name, None)
            .await?;

        match inspect.state {
            None => Ok(TabbyWorkerState::Down),
            Some(state) => match state.status {
                None => Ok(TabbyWorkerState::Down),
                Some(ContainerStateStatusEnum::RUNNING) => Ok(TabbyWorkerState::Up),
                Some(_) => Ok(TabbyWorkerState::Down),
            },
        }
    }
}
