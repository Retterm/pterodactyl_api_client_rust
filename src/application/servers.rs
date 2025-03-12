use crate::application::{Client, structs::CreateServerRequest, structs::CreateServerResponse};
use crate::application::structs::ServerStruct;
use crate::http::EmptyBody;
use crate::structs::{PteroList, PteroObject};
use reqwest::Method;

impl Client {
    /// Lists all servers in the application
    ///
    /// # Example
    ///
    /// ```no_run
    /// use pterodactyl_api_client_rust::application::ClientBuilder;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = ClientBuilder::new("https://pterodactyl.example.com", "your-api-key")
    ///         .build();
    ///
    ///     match client.list_servers().await {
    ///         Ok(servers) => {
    ///             println!("Found {} servers:", servers.len());
    ///             for server in servers {
    ///                 println!("- {} ({})", server.name, server.identifier);
    ///             }
    ///         },
    ///         Err(e) => eprintln!("Error listing servers: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn list_servers(&self) -> crate::Result<Vec<ServerStruct>> {
        self.request::<PteroList<ServerStruct>>(Method::GET, "servers")
            .await
            .map(|servers| servers.data)
    }

    /// Gets a specific server by ID
    ///
    /// # Example
    ///
    /// ```no_run
    /// use pterodactyl_api_client_rust::application::ClientBuilder;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = ClientBuilder::new("https://pterodactyl.example.com", "your-api-key")
    ///         .build();
    ///
    ///     match client.get_server(1).await {
    ///         Ok(server) => println!("Server: {} ({})", server.name, server.identifier),
    ///         Err(e) => eprintln!("Error getting server: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn get_server(&self, id: u32) -> crate::Result<ServerStruct> {
        self.request::<PteroObject<ServerStruct>>(Method::GET, &format!("servers/{}", id))
            .await
            .map(|server| server.attributes)
    }

    /// Creates a new server with the given configuration
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::collections::HashMap;
    /// use pterodactyl_api_client_rust::application::{ClientBuilder, structs::{CreateServerRequest, ServerLimits, ServerFeatureLimits, AllocationSettings}};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = ClientBuilder::new("https://pterodactyl.example.com", "your-api-key")
    ///         .build();
    ///
    ///     let mut environment = HashMap::new();
    ///     environment.insert("SERVER_JARFILE".to_string(), "server.jar".to_string());
    ///     environment.insert("MINECRAFT_VERSION".to_string(), "latest".to_string());
    ///
    ///     let server_request = CreateServerRequest {
    ///         name: "Building".to_string(),
    ///         user: 1,
    ///         egg: 1,
    ///         docker_image: "quay.io/pterodactyl/core:java".to_string(),
    ///         startup: "java -Xms128M -Xmx128M -jar server.jar".to_string(),
    ///         environment,
    ///         limits: ServerLimits {
    ///             memory: 128,
    ///             swap: 0,
    ///             disk: 512,
    ///             io: 500,
    ///             cpu: 100,
    ///             threads: None,
    ///         },
    ///         feature_limits: ServerFeatureLimits {
    ///             databases: 5,
    ///             allocations: 0,
    ///             backups: 1,
    ///         },
    ///         allocation: AllocationSettings {
    ///             default: 17,
    ///         },
    ///     };
    ///
    ///     match client.create_server(server_request).await {
    ///         Ok(server) => println!("Server created: {}", server.attributes.name),
    ///         Err(e) => eprintln!("Error creating server: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn create_server(&self, request: CreateServerRequest) -> crate::Result<CreateServerResponse> {
        self.request_with_body::<PteroObject<CreateServerResponse>, _>(
            Method::POST,
            "servers",
            &request,
        )
        .await
        .map(|response| response.attributes)
    }

    /// Deletes a server with the specified ID
    ///
    /// # Example
    ///
    /// ```no_run
    /// use pterodactyl_api_client_rust::application::ClientBuilder;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = ClientBuilder::new("https://pterodactyl.example.com", "your-api-key")
    ///         .build();
    ///
    ///     match client.delete_server(1).await {
    ///         Ok(_) => println!("Server deleted successfully"),
    ///         Err(e) => eprintln!("Error deleting server: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn delete_server(&self, id: u32) -> crate::Result<()> {
        self.request::<EmptyBody>(Method::DELETE, &format!("servers/{}", id))
            .await?;
        Ok(())
    }

    /// Forcefully deletes a server with the specified ID
    ///
    /// This will delete the server even if it has active backups or other resources.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use pterodactyl_api_client_rust::application::ClientBuilder;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = ClientBuilder::new("https://pterodactyl.example.com", "your-api-key")
    ///         .build();
    ///
    ///     match client.force_delete_server(1).await {
    ///         Ok(_) => println!("Server forcefully deleted successfully"),
    ///         Err(e) => eprintln!("Error forcefully deleting server: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn force_delete_server(&self, id: u32) -> crate::Result<()> {
        self.request::<EmptyBody>(Method::DELETE, &format!("servers/{}/force", id))
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::application::{ClientBuilder, structs::{ServerLimits, ServerFeatureLimits, AllocationSettings}};
    use std::collections::HashMap;

    fn make_test_client() -> Client {
        ClientBuilder::new(
            std::env::var("API_URL").expect("Expected API_URL in environment variables"),
            std::env::var("APPLICATION_API_KEY").expect("Expected APPLICATION_API_KEY in environment variables"),
        )
        .build()
    }

    #[tokio::test]
    async fn test_list_servers() {
        let client = make_test_client();
        let result = client.list_servers().await;
        println!("List servers result: {:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_server() {
        let client = make_test_client();
        // Get the first server from the list to test get_server
        let servers = client.list_servers().await.expect("Failed to list servers");
        if let Some(server) = servers.first() {
            let result = client.get_server(server.id).await;
            println!("Get server result: {:?}", result);
            assert!(result.is_ok());
        } else {
            println!("No servers found to test get_server");
        }
    }

    #[tokio::test]
    #[ignore] // Ignore by default as it creates a real server
    async fn test_create_server() {
        let client = make_test_client();

        let mut environment = HashMap::new();
        environment.insert("SERVER_JARFILE".to_string(), "server.jar".to_string());
        environment.insert("MINECRAFT_VERSION".to_string(), "latest".to_string());

        let server_request = CreateServerRequest {
            name: "Test Server".to_string(),
            user: 1, // Replace with a valid user ID
            egg: 1,  // Replace with a valid egg ID
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
                default: 17, // Replace with a valid allocation ID
            },
        };

        let result = client.create_server(server_request).await;
        println!("Create server result: {:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // Ignore by default as it deletes a real server
    async fn test_delete_server() {
        let client = make_test_client();
        // Replace with a valid server ID to test deletion
        let server_id = 1;
        let result = client.delete_server(server_id).await;
        println!("Delete server result: {:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // Ignore by default as it forcefully deletes a real server
    async fn test_force_delete_server() {
        let client = make_test_client();
        // Replace with a valid server ID to test force deletion
        let server_id = 1;
        let result = client.force_delete_server(server_id).await;
        println!("Force delete server result: {:?}", result);
        assert!(result.is_ok());
    }
} 