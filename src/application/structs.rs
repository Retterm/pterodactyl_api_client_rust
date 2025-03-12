use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a server in the application API
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ServerStruct {
    /// The server's ID
    pub id: u32,
    /// The server's external ID (if any)
    #[serde(default)]
    pub external_id: Option<String>,
    /// The server's UUID
    pub uuid: String,
    /// The server's identifier
    pub identifier: String,
    /// The server's name
    pub name: String,
    /// The server's description
    pub description: String,
    /// Whether the server is suspended
    pub suspended: bool,
    /// The server's resource limits
    pub limits: ServerLimits,
    /// The server's feature limits
    pub feature_limits: ServerFeatureLimits,
    /// The server's user ID
    pub user: u32,
    /// The server's node ID
    pub node: u32,
    /// The server's allocation ID
    pub allocation: u32,
    /// The server's nest ID
    pub nest: u32,
    /// The server's egg ID
    pub egg: u32,
    /// The server's container settings
    pub container: ServerContainer,
    /// The server's updated at timestamp
    pub updated_at: String,
    /// The server's created at timestamp
    pub created_at: String,
}

/// Represents a server's resource limits
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerLimits {
    /// The server's memory limit in MB
    pub memory: u32,
    /// The server's swap limit in MB
    pub swap: u32,
    /// The server's disk limit in MB
    pub disk: u32,
    /// The server's IO limit
    pub io: u32,
    /// The server's CPU limit
    pub cpu: u32,
    /// The server's threads limit (if any)
    #[serde(default)]
    pub threads: Option<u32>,
}

/// Represents a server's feature limits
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerFeatureLimits {
    /// The server's database limit
    pub databases: u32,
    /// The server's allocation limit
    #[serde(default)]
    pub allocations: u32,
    /// The server's backup limit
    pub backups: u32,
}

/// Represents a server's container settings
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ServerContainer {
    /// The server's startup command
    pub startup_command: String,
    /// The server's image
    pub image: String,
    /// The server's installed state
    pub installed: bool,
    /// The server's environment variables
    pub environment: HashMap<String, String>,
}

/// Request body for creating a new server
#[derive(Debug, Serialize)]
pub struct CreateServerRequest {
    /// The server's name
    pub name: String,
    /// The server's user ID
    pub user: u32,
    /// The server's egg ID
    pub egg: u32,
    /// The server's docker image
    pub docker_image: String,
    /// The server's startup command
    pub startup: String,
    /// The server's environment variables
    pub environment: HashMap<String, String>,
    /// The server's resource limits
    pub limits: ServerLimits,
    /// The server's feature limits
    pub feature_limits: ServerFeatureLimits,
    /// The server's allocation settings
    pub allocation: AllocationSettings,
}

/// Allocation settings for server creation
#[derive(Debug, Serialize)]
pub struct AllocationSettings {
    /// The default allocation ID
    pub default: u32,
}

/// Response for server creation
#[derive(Debug, Deserialize)]
pub struct CreateServerResponse {
    /// The created server object
    pub object: String,
    /// The server's attributes
    pub attributes: ServerStruct,
} 