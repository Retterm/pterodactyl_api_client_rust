# Pterodactyl API Client for Rust

A Rust client for the [Pterodactyl API](https://dashflo.net/docs/api/pterodactyl/v1/).

## Features

- Client API (`/api/client`) for user-level operations
- Application API (`/api/application`) for admin-level operations
- Strongly typed API responses
- Async/await support
- Error handling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pterodactyl_api_client_rust = "0.1.0"
```

## Usage

### Client API

The Client API is used for user-level operations, such as managing servers, files, and backups.

```rust
use pterodactyl_api_client_rust::client::ClientBuilder;

#[tokio::main]
async fn main() {
    let client = ClientBuilder::new("https://pterodactyl.example.com", "your-client-api-key")
        .build();

    // List servers
    match client.list_servers().await {
        Ok(servers) => {
            println!("Found {} servers:", servers.len());
            for server in servers {
                println!("- {} ({})", server.name, server.identifier);
            }
        },
        Err(e) => eprintln!("Error listing servers: {}", e),
    }
}
```

### Application API

The Application API is used for admin-level operations, such as creating servers, managing users, and managing nodes.

```rust
use std::collections::HashMap;
use pterodactyl_api_client_rust::application::{ClientBuilder, structs::{CreateServerRequest, ServerLimits, ServerFeatureLimits, AllocationSettings}};

#[tokio::main]
async fn main() {
    let client = ClientBuilder::new("https://pterodactyl.example.com", "your-application-api-key")
        .build();

    // List servers
    match client.list_servers().await {
        Ok(servers) => {
            println!("Found {} servers:", servers.len());
            for server in servers {
                println!("- {} ({})", server.name, server.identifier);
            }
        },
        Err(e) => eprintln!("Error listing servers: {}", e),
    }

    // Create a new server
    let mut environment = HashMap::new();
    environment.insert("SERVER_JARFILE".to_string(), "server.jar".to_string());
    environment.insert("MINECRAFT_VERSION".to_string(), "latest".to_string());

    let server_request = CreateServerRequest {
        name: "Building".to_string(),
        user: 1,
        egg: 1,
        docker_image: "quay.io/pterodactyl/core:java".to_string(),
        startup: "java -Xms128M -Xmx128M -jar server.jar".to_string(),
        environment,
        limits: ServerLimits {
            memory: 128,
            swap: 0,
            disk: 512,
            io: 500,
            cpu: 100,
            threads: None,
        },
        feature_limits: ServerFeatureLimits {
            databases: 5,
            allocations: 0,
            backups: 1,
        },
        allocation: AllocationSettings {
            default: 17,
        },
    };

    match client.create_server(server_request).await {
        Ok(server) => println!("Server created: {}", server.attributes.name),
        Err(e) => eprintln!("Error creating server: {}", e),
    }
    
    // Delete a server
    match client.delete_server(1).await {
        Ok(_) => println!("Server deleted successfully"),
        Err(e) => eprintln!("Error deleting server: {}", e),
    }
    
    // Force delete a server (when normal deletion fails)
    match client.force_delete_server(1).await {
        Ok(_) => println!("Server forcefully deleted successfully"),
        Err(e) => eprintln!("Error forcefully deleting server: {}", e),
    }
}
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
