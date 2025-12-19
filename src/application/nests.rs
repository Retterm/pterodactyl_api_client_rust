use crate::application::structs::{EggStruct, NestStruct};
use crate::application::Client;
use crate::structs::{PteroList, PteroObject};
use reqwest::Method;

impl Client {
    /// Lists all nests in the application
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
    ///     match client.list_nests().await {
    ///         Ok(nests) => {
    ///             println!("Found {} nests:", nests.len());
    ///             for nest in nests {
    ///                 println!("- {} ({})", nest.name, nest.id);
    ///             }
    ///         },
    ///         Err(e) => eprintln!("Error listing nests: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn list_nests(&self) -> crate::Result<Vec<NestStruct>> {
        self.request::<PteroList<NestStruct>>(Method::GET, "nests")
            .await
            .map(|nests| nests.data)
    }

    /// Gets a specific nest by ID
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
    ///     match client.get_nest(1).await {
    ///         Ok(nest) => println!("Nest: {} ({})", nest.name, nest.id),
    ///         Err(e) => eprintln!("Error getting nest: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn get_nest(&self, id: u32) -> crate::Result<NestStruct> {
        self.request::<PteroObject<NestStruct>>(Method::GET, &format!("nests/{}", id))
            .await
            .map(|nest| nest.attributes)
    }

    /// Lists all eggs in a specific nest
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
    ///     match client.list_eggs(1).await {
    ///         Ok(eggs) => {
    ///             println!("Found {} eggs in nest 1:", eggs.len());
    ///             for egg in eggs {
    ///                 println!("- {} ({})", egg.name, egg.id);
    ///             }
    ///         },
    ///         Err(e) => eprintln!("Error listing eggs: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn list_eggs(
        &self,
        nest_id: u32,
        include: Option<Vec<&str>>,
    ) -> crate::Result<Vec<EggStruct>> {
        let endpoint = match include {
            Some(includes) if !includes.is_empty() => {
                let include_str = includes.join(",");
                format!("nests/{}/eggs?include={}", nest_id, include_str)
            }
            _ => format!("nests/{}/eggs", nest_id),
        };
        self.request::<PteroList<EggStruct>>(Method::GET, &endpoint)
            .await
            .map(|eggs| eggs.data)
    }

    /// Gets a specific egg by nest ID and egg ID
    ///
    /// The `include` parameter allows you to include additional related resources in the response.
    /// For example, you can include the nest, servers, config, or variables.
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
    ///     // Get egg without includes
    ///     match client.get_egg(1, 1, None).await {
    ///         Ok(egg) => println!("Egg: {} ({})", egg.name, egg.id),
    ///         Err(e) => eprintln!("Error getting egg: {}", e),
    ///     }
    ///
    ///     // Include variables in the response
    ///     match client.get_egg(1, 1, Some(vec!["variables", "config"])).await {
    ///         Ok(egg) => println!("Egg: {} ({}) with variables", egg.name, egg.id),
    ///         Err(e) => eprintln!("Error getting egg: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn get_egg(
        &self,
        nest_id: u32,
        egg_id: u32,
        include: Option<Vec<&str>>,
    ) -> crate::Result<EggStruct> {
        let endpoint = match include {
            Some(includes) if !includes.is_empty() => {
                let include_str = includes.join(",");
                format!("nests/{}/eggs/{}?include={}", nest_id, egg_id, include_str)
            }
            _ => format!("nests/{}/eggs/{}", nest_id, egg_id),
        };

        self.request::<PteroObject<EggStruct>>(Method::GET, &endpoint)
            .await
            .map(|egg| egg.attributes)
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
    async fn test_list_nests() {
        let client = make_test_client();
        let result = client.list_nests().await;
        println!("List nests result: {:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_nest() {
        let client = make_test_client();
        // Get the first nest from the list to test get_nest
        let nests = client.list_nests().await.expect("Failed to list nests");
        if let Some(nest) = nests.first() {
            let result = client.get_nest(nest.id).await;
            println!("Get nest result: {:?}", result);
            assert!(result.is_ok());
        } else {
            println!("No nests found to test get_nest");
        }
    }

    #[tokio::test]
    async fn test_list_eggs() {
        let client = make_test_client();
        // Get the first nest from the list to test list_eggs
        let nests = client.list_nests().await.expect("Failed to list nests");
        if let Some(nest) = nests.first() {
            let result = client.list_eggs(nest.id, None).await;
            println!("List eggs result: {:?}", result);
            assert!(result.is_ok());
        } else {
            println!("No nests found to test list_eggs");
        }
    }

    #[tokio::test]
    async fn test_get_egg() {
        let client = make_test_client();
        // Get the first nest from the list
        let nests = client.list_nests().await.expect("Failed to list nests");
        if let Some(nest) = nests.first() {
            // Get the first egg from the nest
            let eggs = client
                .list_eggs(nest.id, None)
                .await
                .expect("Failed to list eggs");
            if let Some(egg) = eggs.first() {
                let result = client.get_egg(nest.id, egg.id, None).await;
                println!("Get egg result: {:?}", result);
                assert!(result.is_ok());
            } else {
                println!("No eggs found to test get_egg");
            }
        } else {
            println!("No nests found to test get_egg");
        }
    }

    #[tokio::test]
    async fn test_get_egg_with_includes() {
        let client = make_test_client();
        // Get the first nest from the list
        let nests = client.list_nests().await.expect("Failed to list nests");
        if let Some(nest) = nests.first() {
            // Get the first egg from the nest
            let eggs = client
                .list_eggs(nest.id, None)
                .await
                .expect("Failed to list eggs");
            if let Some(egg) = eggs.first() {
                // Test with variables included
                let result = client
                    .get_egg(nest.id, egg.id, Some(vec!["variables"]))
                    .await;
                println!("Get egg with variables result: {:?}", result);
                assert!(result.is_ok());

                // Test with both config and variables included
                let result = client
                    .get_egg(nest.id, egg.id, Some(vec!["config", "variables"]))
                    .await;
                println!("Get egg with config and variables result: {:?}", result);
                assert!(result.is_ok());
            } else {
                println!("No eggs found to test get_egg_with_includes");
            }
        } else {
            println!("No nests found to test get_egg_with_includes");
        }
    }
}
