/// Tabby worker types
#[derive(Debug, Clone)]
pub enum TabbyWorkerType {
    /// Worker is used for chat
    Chat,
    /// Worker is used for code completion
    Completion,
}

/// Tabby device types
///
/// We don't care what really passes to --device option, because container handles it
#[derive(Debug, Clone)]
pub enum TabbyDeviceType {
    /// Tabby worker is using CPU
    CPU,
    /// Tabby worker is using GPU
    GPU,
}

/// Tells how to handle worker
#[derive(Debug, Clone)]
pub enum TabbyWorkerHandler {
    /// Use docker to start up worker
    Docker(String),
}

/// State of worker
#[derive(Debug, Clone)]
pub enum TabbyWorkerState {
    /// Worked is offline or down
    Down,
    /// Worker is up and running
    Up,
}

/// Tabby worker description required to configure it
#[derive(Debug, Clone)]
pub struct TabbyWorkerConfig {
    /// Name of worker, must be unique
    pub name: String,
    /// Type of worker
    pub worker_type: TabbyWorkerType,
    /// Type of device worker is using
    pub device_type: TabbyDeviceType,

    /// Handler for worker
    pub handler: TabbyWorkerHandler,
}
