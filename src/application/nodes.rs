use crate::application::structs::{
    AllocationStruct, CreateAllocationRequest, CreateNodeRequest, NodeStruct, UpdateNodeRequest,
};
use crate::application::Client;
use crate::http::EmptyBody;
use crate::structs::{PteroList, PteroObject};
use reqwest::Method;

impl Client {
    /// Lists all nodes in the application
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
    ///     match client.list_nodes().await {
    ///         Ok(nodes) => {
    ///             println!("Found {} nodes:", nodes.len());
    ///             for node in nodes {
    ///                 println!("- {} ({})", node.name, node.id);
    ///             }
    ///         },
    ///         Err(e) => eprintln!("Error listing nodes: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn list_nodes(&self) -> crate::Result<Vec<NodeStruct>> {
        self.request::<PteroList<NodeStruct>>(Method::GET, "nodes")
            .await
            .map(|nodes| nodes.data)
    }

    /// Gets a specific node by ID
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
    ///     match client.get_node(1).await {
    ///         Ok(node) => println!("Node: {} ({})", node.name, node.id),
    ///         Err(e) => eprintln!("Error getting node: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn get_node(&self, id: u32) -> crate::Result<NodeStruct> {
        self.request::<PteroObject<NodeStruct>>(Method::GET, &format!("nodes/{}", id))
            .await
            .map(|node| node.attributes)
    }

    /// Creates a new node with the given configuration
    ///
    /// # Example
    ///
    /// ```no_run
    /// use pterodactyl_api_client_rust::application::{ClientBuilder, structs::CreateNodeRequest};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = ClientBuilder::new("https://pterodactyl.example.com", "your-api-key")
    ///         .build();
    ///
    ///     let node_request = CreateNodeRequest {
    ///         name: "New Node".to_string(),
    ///         description: Some("A new node for testing".to_string()),
    ///         location_id: 1,
    ///         public: Some(true),
    ///         fqdn: "node.example.com".to_string(),
    ///         scheme: "https".to_string(),
    ///         behind_proxy: Some(false),
    ///         memory: 4096,
    ///         memory_overallocate: 0,
    ///         disk: 10000,
    ///         disk_overallocate: 0,
    ///         daemon_base: Some("/srv/daemon-data".to_string()),
    ///         daemon_sftp: 2022,
    ///         daemon_listen: 8080,
    ///         maintenance_mode: Some(false),
    ///         upload_size: Some(100),
    ///     };
    ///
    ///     match client.create_node(node_request).await {
    ///         Ok(node) => println!("Node created: {} ({})", node.name, node.id),
    ///         Err(e) => eprintln!("Error creating node: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn create_node(&self, request: CreateNodeRequest) -> crate::Result<NodeStruct> {
        self.request_with_body::<PteroObject<NodeStruct>, _>(Method::POST, "nodes", &request)
            .await
            .map(|response| response.attributes)
    }

    /// Updates a node with the specified ID
    ///
    /// # Example
    ///
    /// ```no_run
    /// use pterodactyl_api_client_rust::application::{ClientBuilder, structs::UpdateNodeRequest};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = ClientBuilder::new("https://pterodactyl.example.com", "your-api-key")
    ///         .build();
    ///
    ///     let update_request = UpdateNodeRequest {
    ///         name: Some("Updated Node".to_string()),
    ///         description: Some("An updated node description".to_string()),
    ///         location_id: None,
    ///         public: None,
    ///         fqdn: None,
    ///         scheme: None,
    ///         behind_proxy: None,
    ///         memory: None,
    ///         memory_overallocate: None,
    ///         disk: None,
    ///         disk_overallocate: None,
    ///         daemon_base: None,
    ///         daemon_sftp: None,
    ///         daemon_listen: None,
    ///         maintenance_mode: Some(true),
    ///         upload_size: None,
    ///     };
    ///
    ///     match client.update_node(1, update_request).await {
    ///         Ok(node) => println!("Node updated: {} ({})", node.name, node.id),
    ///         Err(e) => eprintln!("Error updating node: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn update_node(
        &self,
        id: u32,
        request: UpdateNodeRequest,
    ) -> crate::Result<NodeStruct> {
        self.request_with_body::<PteroObject<NodeStruct>, _>(
            Method::PATCH,
            &format!("nodes/{}", id),
            &request,
        )
        .await
        .map(|response| response.attributes)
    }

    /// Deletes a node with the specified ID
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
    ///     match client.delete_node(1).await {
    ///         Ok(_) => println!("Node deleted successfully"),
    ///         Err(e) => eprintln!("Error deleting node: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn delete_node(&self, id: u32) -> crate::Result<()> {
        self.request::<EmptyBody>(Method::DELETE, &format!("nodes/{}", id))
            .await?;
        Ok(())
    }

    /// Lists all allocations for a specific node
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
    ///     match client.list_node_allocations(1).await {
    ///         Ok(allocations) => {
    ///             println!("Found {} allocations for node 1:", allocations.len());
    ///             for allocation in allocations {
    ///                 println!("- {}:{} ({})", allocation.ip, allocation.port, allocation.id);
    ///             }
    ///         },
    ///         Err(e) => eprintln!("Error listing allocations: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn list_node_allocations(
        &self,
        node_id: u32,
    ) -> crate::Result<Vec<AllocationStruct>> {
        self.request::<PteroList<AllocationStruct>>(
            Method::GET,
            &format!("nodes/{}/allocations", node_id),
        )
        .await
        .map(|allocations| allocations.data)
    }

    /// Creates a new allocation for a specific node
    ///
    /// # Example
    ///
    /// ```no_run
    /// use pterodactyl_api_client_rust::application::{ClientBuilder, structs::CreateAllocationRequest};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = ClientBuilder::new("https://pterodactyl.example.com", "your-api-key")
    ///         .build();
    ///
    ///     let allocation_request = CreateAllocationRequest {
    ///         ip: "192.168.1.1".to_string(),
    ///         ports: vec!["25565".to_string(), "25566-25570".to_string()],
    ///         alias: Some("Minecraft".to_string()),
    ///     };
    ///
    ///     match client.create_node_allocation(1, allocation_request).await {
    ///         Ok(_) => println!("Allocations created successfully"),
    ///         Err(e) => eprintln!("Error creating allocations: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn create_node_allocation(
        &self,
        node_id: u32,
        request: CreateAllocationRequest,
    ) -> crate::Result<()> {
        self.request_with_body::<EmptyBody, _>(
            Method::POST,
            &format!("nodes/{}/allocations", node_id),
            &request,
        )
        .await?;
        Ok(())
    }

    /// Deletes an allocation with the specified ID
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
    ///     match client.delete_allocation(1).await {
    ///         Ok(_) => println!("Allocation deleted successfully"),
    ///         Err(e) => eprintln!("Error deleting allocation: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn delete_allocation(&self, allocation_id: u32) -> crate::Result<()> {
        self.request::<EmptyBody>(Method::DELETE, &format!("allocations/{}", allocation_id))
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::application::ClientBuilder;

    fn make_test_client() -> Client {
        ClientBuilder::new(
            std::env::var("API_URL").expect("Expected API_URL in environment variables"),
            std::env::var("APPLICATION_API_KEY")
                .expect("Expected APPLICATION_API_KEY in environment variables"),
        )
        .build()
    }

    #[tokio::test]
    async fn test_list_nodes() {
        let client = make_test_client();
        let result = client.list_nodes().await;
        println!("List nodes result: {:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_node() {
        let client = make_test_client();
        // Get the first node from the list to test get_node
        let nodes = client.list_nodes().await.expect("Failed to list nodes");
        if let Some(node) = nodes.first() {
            let result = client.get_node(node.id).await;
            println!("Get node result: {:?}", result);
            assert!(result.is_ok());
        } else {
            println!("No nodes found to test get_node");
        }
    }

    #[tokio::test]
    #[ignore] // Ignore by default as it creates a real node
    async fn test_create_node() {
        let client = make_test_client();

        let node_request = CreateNodeRequest {
            name: "Test Node".to_string(),
            description: Some("A test node".to_string()),
            location_id: 1, // Replace with a valid location ID
            public: Some(true),
            fqdn: "test.example.com".to_string(),
            scheme: "https".to_string(),
            behind_proxy: Some(false),
            memory: 4096,
            memory_overallocate: 0,
            disk: 10000,
            disk_overallocate: 0,
            daemon_base: Some("/srv/daemon-data".to_string()),
            daemon_sftp: 2022,
            daemon_listen: 8080,
            maintenance_mode: Some(false),
            upload_size: Some(100),
        };

        let result = client.create_node(node_request).await;
        println!("Create node result: {:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // Ignore by default as it updates a real node
    async fn test_update_node() {
        let client = make_test_client();
        // Get the first node from the list to test update_node
        let nodes = client.list_nodes().await.expect("Failed to list nodes");
        if let Some(node) = nodes.first() {
            let update_request = UpdateNodeRequest {
                name: Some(format!("{} (Updated)", node.name)),
                description: Some("Updated description".to_string()),
                location_id: None,
                public: None,
                fqdn: None,
                scheme: None,
                behind_proxy: None,
                memory: None,
                memory_overallocate: None,
                disk: None,
                disk_overallocate: None,
                daemon_base: None,
                daemon_sftp: None,
                daemon_listen: None,
                maintenance_mode: None,
                upload_size: None,
            };

            let result = client.update_node(node.id, update_request).await;
            println!("Update node result: {:?}", result);
            assert!(result.is_ok());
        } else {
            println!("No nodes found to test update_node");
        }
    }

    #[tokio::test]
    #[ignore] // Ignore by default as it deletes a real node
    async fn test_delete_node() {
        let client = make_test_client();
        // Replace with a valid node ID to test deletion
        let node_id = 1;
        let result = client.delete_node(node_id).await;
        println!("Delete node result: {:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_node_allocations() {
        let client = make_test_client();
        // Get the first node from the list to test list_node_allocations
        let nodes = client.list_nodes().await.expect("Failed to list nodes");
        if let Some(node) = nodes.first() {
            let result = client.list_node_allocations(node.id).await;
            println!("List node allocations result: {:?}", result);
            assert!(result.is_ok());
        } else {
            println!("No nodes found to test list_node_allocations");
        }
    }

    #[tokio::test]
    #[ignore] // Ignore by default as it creates real allocations
    async fn test_create_node_allocation() {
        let client = make_test_client();
        // Get the first node from the list to test create_node_allocation
        let nodes = client.list_nodes().await.expect("Failed to list nodes");
        if let Some(node) = nodes.first() {
            let allocation_request = CreateAllocationRequest {
                ip: "192.168.1.1".to_string(), // Replace with a valid IP
                ports: vec!["25565".to_string(), "25566-25570".to_string()],
                alias: Some("Test Allocation".to_string()),
            };

            let result = client
                .create_node_allocation(node.id, allocation_request)
                .await;
            println!("Create node allocation result: {:?}", result);
            assert!(result.is_ok());
        } else {
            println!("No nodes found to test create_node_allocation");
        }
    }

    #[tokio::test]
    #[ignore] // Ignore by default as it deletes a real allocation
    async fn test_delete_allocation() {
        let client = make_test_client();
        // Replace with a valid allocation ID to test deletion
        let allocation_id = 1;
        let result = client.delete_allocation(allocation_id).await;
        println!("Delete allocation result: {:?}", result);
        assert!(result.is_ok());
    }
}
