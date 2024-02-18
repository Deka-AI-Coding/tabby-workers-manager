mod error;
pub use error::Error;
pub use error::Result;

mod worker;
pub use worker::TabbyDeviceType;
pub use worker::TabbyWorkerConfig;
pub use worker::TabbyWorkerHandler;
pub use worker::TabbyWorkerState;
pub use worker::TabbyWorkerType;

mod manager;
use manager::Manager;
use manager::ManagerFactory;

pub fn default_config() -> Vec<TabbyWorkerConfig> {
    vec![
        TabbyWorkerConfig {
            name: "tabby-chat-cpu".to_string(),
            worker_type: TabbyWorkerType::Chat,
            device_type: TabbyDeviceType::CPU,
            handler: TabbyWorkerHandler::Docker("tabby-chat-cpu".to_string()),
        },
        TabbyWorkerConfig {
            name: "tabby-chat-gpu".to_string(),
            worker_type: TabbyWorkerType::Chat,
            device_type: TabbyDeviceType::GPU,
            handler: TabbyWorkerHandler::Docker("tabby-chat-gpu".to_string()),
        },
        TabbyWorkerConfig {
            name: "tabby-completion-cpu".to_string(),
            worker_type: TabbyWorkerType::Completion,
            device_type: TabbyDeviceType::CPU,
            handler: TabbyWorkerHandler::Docker("tabby-completion-cpu".to_string()),
        },
        TabbyWorkerConfig {
            name: "tabby-completion-gpu".to_string(),
            worker_type: TabbyWorkerType::Completion,
            device_type: TabbyDeviceType::GPU,
            handler: TabbyWorkerHandler::Docker("tabby-completion-gpu".to_string()),
        },
    ]
}

/// Controls one worker
pub struct TabbyWorkerManager {
    manager: Box<dyn Manager>,
}

impl TabbyWorkerManager {
    fn new(config: TabbyWorkerConfig) -> Self {
        let factory = ManagerFactory::default();
        let manager = factory.create_manager(config);
        Self { manager }
    }

    /// Starts associated worker
    pub async fn start(&self) -> Result<()> {
        self.manager.start().await
    }

    /// Stops associated worker
    pub async fn stop(&self) -> Result<()> {
        self.manager.stop().await
    }

    /// Returns worker's state
    pub async fn state(&self) -> Result<TabbyWorkerState> {
        self.manager.state().await
    }
}

/// Holds all workers and manages them
pub struct TabbyWorkerZookeeper {
    /// Workers
    workers: Vec<TabbyWorkerConfig>,
}

impl Default for TabbyWorkerZookeeper {
    fn default() -> Self {
        Self {
            workers: default_config(),
        }
    }
}

impl TabbyWorkerZookeeper {
    /// Returns worker manager by name of worker
    pub async fn manager(&self, name: &str) -> Result<TabbyWorkerManager> {
        self.workers
            .iter()
            .find(|w| w.name == name)
            .ok_or(Error::WorkerNotFound(name.to_string()))
            .cloned()
            .map(TabbyWorkerManager::new)
    }
}
