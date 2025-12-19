use std::collections::HashMap;
use urlencoding::encode;

use crate::application::structs::ServerStruct;
use crate::application::{structs::CreateServerRequest, structs::CreateServerResponse, Client};
use crate::http::EmptyBody;
use crate::structs::{PteroList, PteroObject};
use reqwest::Method;
use serde::{Deserialize, Serialize};

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

    /// Lists servers with query parameters (filters/pagination/include/sort)
    ///
    /// This wraps GET /api/application/servers with optional query parameters supported by
    /// the latest Pterodactyl API, such as `filter[uuid]`, `filter[name]`, `filter[external_id]`,
    /// `per_page`, `page`, `include`, and `sort`.
    pub async fn list_servers_filtered(
        &self,
        filter_name: Option<&str>,
        filter_uuid: Option<&str>,
        filter_external_id: Option<&str>,
        filter_image: Option<&str>,
        include: Option<&[&str]>,
        sort: Option<&str>,
        per_page: Option<u32>,
        page: Option<u32>,
    ) -> crate::Result<Vec<ServerStruct>> {
        let mut pairs: Vec<String> = Vec::new();

        if let Some(v) = filter_name {
            pairs.push(format!("{}={}", encode("filter[name]"), encode(v)));
        }
        if let Some(v) = filter_uuid {
            pairs.push(format!("{}={}", encode("filter[uuid]"), encode(v)));
        }
        if let Some(v) = filter_external_id {
            pairs.push(format!("{}={}", encode("filter[external_id]"), encode(v)));
        }
        if let Some(v) = filter_image {
            pairs.push(format!("{}={}", encode("filter[image]"), encode(v)));
        }
        if let Some(v) = include {
            if !v.is_empty() {
                pairs.push(format!("{}={}", encode("include"), encode(&v.join(","))));
            }
        }
        if let Some(v) = sort {
            pairs.push(format!("{}={}", encode("sort"), encode(v)));
        }
        if let Some(v) = per_page {
            pairs.push(format!("{}={}", encode("per_page"), encode(&v.to_string())));
        }
        if let Some(v) = page {
            pairs.push(format!("{}={}", encode("page"), encode(&v.to_string())));
        }

        let qs = pairs.join("&");
        let endpoint = if qs.is_empty() {
            "servers".to_string()
        } else {
            format!("servers?{}", qs)
        };

        self.request::<PteroList<ServerStruct>>(Method::GET, &endpoint)
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
    pub async fn create_server(
        &self,
        request: CreateServerRequest,
    ) -> crate::Result<CreateServerResponse> {
        self.request_with_body::<CreateServerResponse, _>(Method::POST, "servers", &request)
            .await
            .map(|response| response)
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

    /// Updates server build limits (CPU/Memory/Disk/IO/Swap/Threads)
    pub async fn update_server_build(
        &self,
        id: u32,
        limits: crate::application::structs::ServerLimits,
    ) -> crate::Result<()> {
        #[derive(Serialize)]
        struct UpdateBuildBody {
            memory: u32,
            swap: u32,
            disk: u32,
            io: u32,
            cpu: u32,
            allocation: u32,
            feature_limits: crate::application::structs::ServerFeatureLimits,
            #[serde(skip_serializing_if = "Option::is_none")]
            threads: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            oom_disabled: Option<bool>,
        }

        // 获取当前服务器以填充必填但未变更的字段
        let current = self.get_server(id).await?;

        let body = UpdateBuildBody {
            memory: limits.memory,
            swap: limits.swap,
            disk: limits.disk,
            io: limits.io,
            cpu: limits.cpu,
            allocation: current.allocation,
            feature_limits: current.feature_limits,
            threads: limits.threads,
            oom_disabled: limits.oom_disabled,
        };

        self.request_with_body::<EmptyBody, _>(
            Method::PATCH,
            &format!("servers/{}/build", id),
            &body,
        )
        .await?;
        Ok(())
    }

    /// Updates the startup variables for a server
    ///
    /// # Example
    ///
    /// ```no_run
    /// use pterodactyl_api_client_rust::application::ClientBuilder;
    ///
    /// ```no_run
    /// let client = ClientBuilder::new("https://pterodactyl.example.com", "your-api-key")
    ///     .build();
    ///
    /// let startup = "java -Xms128M -Xmx128M -jar server.jar".to_string();
    /// let environment = HashMap::new();
    /// let egg = "1".to_string();
    /// let image = "quay.io/pterodactyl/core:java".to_string();
    /// let skip_scripts = false;
    ///
    /// let result = client.update_startup_variables(1, startup, environment, egg, image, skip_scripts).await;
    /// ```
    pub async fn update_startup_variables(
        &self,
        id: u32,
        startup: String,
        environment: HashMap<String, String>,
        egg: String,
        image: String,
        skip_scripts: bool,
    ) -> crate::Result<()> {
        let body = UpdateStartupVariablesRequest {
            startup,
            environment,
            egg,
            image,
            skip_scripts,
        };
        self.request_with_body::<EmptyBody, _>(
            Method::PATCH,
            &format!("servers/{}/startup", id),
            &body,
        )
        .await?;
        Ok(())
    }

    /// Suspends a server with the specified ID
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
    ///     match client.suspend_server(1).await {
    ///         Ok(_) => println!("Server suspended successfully"),
    ///         Err(e) => eprintln!("Error suspending server: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn suspend_server(&self, id: u32) -> crate::Result<()> {
        self.request::<EmptyBody>(Method::POST, &format!("servers/{}/suspend", id))
            .await?;
        Ok(())
    }

    /// Resumes a server with the specified ID
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
    ///     match client.resume_server(1).await {
    ///         Ok(_) => println!("Server resumed successfully"),
    ///         Err(e) => eprintln!("Error resuming server: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn resume_server(&self, id: u32) -> crate::Result<()> {
        self.request::<EmptyBody>(Method::POST, &format!("servers/{}/resume", id))
            .await?;
        Ok(())
    }
}

///
///
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateStartupVariablesRequest {
    startup: String,
    environment: HashMap<String, String>,
    egg: String,
    image: String,
    skip_scripts: bool,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::application::{
        structs::{AllocationSettings, ServerFeatureLimits, ServerLimits},
        ClientBuilder,
    };
    use std::collections::HashMap;

    fn make_test_client() -> Client {
        ClientBuilder::new(
            std::env::var("API_URL").expect("Expected API_URL in environment variables"),
            std::env::var("APPLICATION_API_KEY")
                .expect("Expected APPLICATION_API_KEY in environment variables"),
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
                oom_disabled: None,
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
