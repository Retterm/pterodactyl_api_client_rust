use serde::{Deserialize, Serialize};

/// Represents an allocation in the application API
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AllocationStruct {
    /// The allocation's ID
    pub id: u32,
    /// The allocation's node ID
    pub node: Option<u32>,
    /// The allocation's IP address
    pub ip: String,
    /// The allocation's IP alias (if any)
    #[serde(default)]
    pub alias: Option<String>,
    /// The allocation's port
    pub port: u16,
    /// Whether the allocation is assigned to a server
    pub assigned: bool,
    /// The allocation's notes (if any
    #[serde(default)]
    pub notes: Option<String>,
}

/// Allocation settings for server creation
#[derive(Debug, Serialize)]
pub struct AllocationSettings {
    /// The default allocation ID
    pub default: u32,
}

/// Request body for creating a new allocation
#[derive(Debug, Serialize)]
pub struct CreateAllocationRequest {
    /// The allocation's IP address
    pub ip: String,
    /// The allocation's ports (can be a range like "25565-25570" or a single port like "25565")
    pub ports: Vec<String>,
    /// The allocation's alias (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}
