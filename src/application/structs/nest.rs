use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a nest in the application API
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct NestStruct {
    /// The nest's ID
    pub id: u32,
    /// The nest's UUID
    pub uuid: String,
    /// The nest's author
    pub author: String,
    /// The nest's name
    pub name: String,
    /// The nest's description
    pub description: String,
    /// The nest's created at timestamp
    pub created_at: String,
    /// The nest's updated at timestamp
    pub updated_at: String,
}

/// Represents an egg in the application API
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct EggStruct {
    /// The egg's ID
    pub id: u32,
    /// The egg's UUID
    pub uuid: String,
    /// The egg's name
    pub name: String,
    /// The egg's nest ID
    pub nest: u32,
    /// The egg's author
    pub author: String,
    /// The egg's description
    pub description: String,
    /// The egg's docker image
    pub docker_image: String,
    /// The egg's docker images
    #[serde(default)]
    pub docker_images: HashMap<String, String>,
    /// The egg's configuration
    pub config: EggConfig,
    /// The egg's startup command
    pub startup: String,
    /// The egg's script information
    pub script: EggScript,
    /// The egg's created at timestamp
    pub created_at: String,
    /// The egg's updated at timestamp
    pub updated_at: String,
    /// The egg's relationships (only included when requested)
    #[serde(default)]
    pub relationships: Option<EggRelationships>,
}

/// Represents an egg's configuration
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct EggConfig {
    /// The egg's configuration files
    pub files: serde_json::Value,
    /// The egg's startup configuration
    pub startup: serde_json::Value,
    /// The egg's stop configuration
    pub stop: String,
    /// The egg's logs configuration
    pub logs: serde_json::Value,
    /// The egg's file denylist
    pub file_denylist: Vec<String>,
    /// The egg's extends configuration
    pub extends: Option<serde_json::Value>,
}

/// Represents an egg's script information
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct EggScript {
    /// Whether the script is privileged
    pub privileged: bool,
    /// The script's install command
    pub install: String,
    /// The script's entry command
    pub entry: String,
    /// The script's container command
    pub container: String,
    /// The script's extends configuration
    pub extends: Option<serde_json::Value>,
}

/// Represents an egg's relationships
#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct EggRelationships {
    /// The egg's configuration relationship
    #[serde(default)]
    pub config: Option<NullResource>,
    /// The egg's variables relationship
    #[serde(default)]
    pub variables: Option<EggVariablesList>,
}

/// Represents a null resource in relationships
#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct NullResource {
    /// The object type
    pub object: String,
    /// The attributes (null for null resources)
    pub attributes: Option<serde_json::Value>,
}

/// Represents a list of egg variables
#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct EggVariablesList {
    /// The object type
    pub object: String,
    /// The list of variables
    pub data: Vec<EggVariableObject>,
}

/// Represents an egg variable object
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct EggVariableObject {
    /// The object type
    pub object: String,
    /// The variable attributes
    pub attributes: EggVariable,
}

/// Represents an egg variable
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct EggVariable {
    /// The variable's ID
    pub id: u32,
    /// The variable's egg ID
    pub egg_id: u32,
    /// The variable's name
    pub name: String,
    /// The variable's description
    pub description: String,
    /// The variable's environment variable name
    pub env_variable: String,
    /// The variable's default value
    pub default_value: String,
    /// Whether the variable is user viewable
    pub user_viewable: bool,
    /// Whether the variable is user editable
    pub user_editable: bool,
    /// The variable's validation rules
    pub rules: String,
    /// The variable's created at timestamp
    pub created_at: String,
    /// The variable's updated at timestamp
    pub updated_at: String,
}
