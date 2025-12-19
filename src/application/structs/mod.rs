// Re-export all structs from submodules
mod allocation;
mod nest;
mod node;
mod server;
mod utils;

// Server-related structs
pub use server::{
    CreateServerRequest, CreateServerResponse, ServerContainer, ServerFeatureLimits, ServerLimits,
    ServerStruct,
};

// Node-related structs
pub use node::{CreateNodeRequest, NodeStruct, UpdateNodeRequest};

// Nest and Egg-related structs
pub use nest::{
    EggConfig, EggRelationships, EggScript, EggStruct, EggVariable, EggVariableObject,
    EggVariablesList, NestStruct, NullResource,
};

// Allocation-related structs
pub use allocation::{AllocationSettings, AllocationStruct, CreateAllocationRequest};

// Utility functions
pub use utils::deserialize_installed;
