use serde::{Deserialize, Serialize};

/// Represents a node in the application API
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct NodeStruct {
    /// The node's ID
    pub id: u32,
    /// Whether the node is public (allows auto-deployment)
    pub public: bool,
    /// The node's name
    pub name: String,
    /// The node's description
    #[serde(default)]
    pub description: Option<String>,
    /// The node's location ID
    pub location_id: u32,
    /// The node's fully qualified domain name
    pub fqdn: String,
    /// The node's connection scheme (http or https)
    pub scheme: String,
    /// Whether the node is behind a proxy
    pub behind_proxy: bool,
    /// Whether the node is in maintenance mode
    pub maintenance_mode: bool,
    /// The node's memory limit in MB
    pub memory: u32,
    /// The node's memory over-allocation percentage
    pub memory_overallocate: i32,
    /// The node's disk space in MB
    pub disk: u32,
    /// The node's disk space over-allocation percentage
    pub disk_overallocate: i32,
    /// The node's upload size limit in MB
    pub upload_size: u32,
    /// The node's daemon listen port
    pub daemon_listen: u16,
    /// The node's daemon SFTP port
    pub daemon_sftp: u16,
    /// The node's daemon base directory
    pub daemon_base: String,
    /// The node's created at timestamp
    pub created_at: String,
    /// The node's updated at timestamp
    pub updated_at: String,
}

/// Request body for creating a new node
#[derive(Debug, Serialize)]
pub struct CreateNodeRequest {
    /// The node's name
    pub name: String,
    /// The node's description (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The node's location ID
    pub location_id: u32,
    /// Whether the node is public (allows auto-deployment)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    /// The node's fully qualified domain name
    pub fqdn: String,
    /// The node's connection scheme (http or https)
    pub scheme: String,
    /// Whether the node is behind a proxy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behind_proxy: Option<bool>,
    /// The node's memory limit in MB
    pub memory: u32,
    /// The node's memory over-allocation percentage
    pub memory_overallocate: i32,
    /// The node's disk space in MB
    pub disk: u32,
    /// The node's disk space over-allocation percentage
    pub disk_overallocate: i32,
    /// The node's daemon base directory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_base: Option<String>,
    /// The node's daemon SFTP port
    pub daemon_sftp: u16,
    /// The node's daemon listen port
    pub daemon_listen: u16,
    /// Whether the node is in maintenance mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_mode: Option<bool>,
    /// The node's upload size limit in MB
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_size: Option<u32>,
}

/// Request body for updating a node
#[derive(Debug, Serialize)]
pub struct UpdateNodeRequest {
    /// The node's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The node's description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The node's location ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<u32>,
    /// Whether the node is public (allows auto-deployment)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    /// The node's fully qualified domain name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    /// The node's connection scheme (http or https)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    /// Whether the node is behind a proxy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behind_proxy: Option<bool>,
    /// The node's memory limit in MB
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<u32>,
    /// The node's memory over-allocation percentage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_overallocate: Option<i32>,
    /// The node's disk space in MB
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<u32>,
    /// The node's disk space over-allocation percentage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_overallocate: Option<i32>,
    /// The node's daemon base directory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_base: Option<String>,
    /// The node's daemon SFTP port
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_sftp: Option<u16>,
    /// The node's daemon listen port
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_listen: Option<u16>,
    /// Whether the node is in maintenance mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_mode: Option<bool>,
    /// The node's upload size limit in MB
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_size: Option<u32>,
}
