use crate::completion::CompletionModel;
use rig::client::ProviderValue;
use rig::impl_conversion_traits;
use rig::prelude::*;
use std::sync::Arc;

/// Default Azure AI Foundry endpoint pattern
pub const DEFAULT_ENDPOINT_PATTERN: &str = "https://{account}.services.ai.azure.com";

#[derive(Clone, Debug)]
pub struct Client {
    api_key: String,
    endpoint: String,
    client: Arc<reqwest::Client>,
}

impl Client {
    /// Create a new Azure AI Foundry client with API key and endpoint
    pub fn new(api_key: String, endpoint: String) -> Self {
        Self {
            api_key,
            endpoint,
            client: Arc::new(reqwest::Client::new()),
        }
    }

    /// Create a client from environment variables
    /// 
    /// Required environment variables:
    /// - AZURE_API_KEY
    /// - AZURE_ENDPOINT
    pub fn from_env() -> Self {
        let api_key = std::env::var("AZURE_API_KEY")
            .expect("AZURE_API_KEY environment variable not set");
        
        let endpoint = std::env::var("AZURE_ENDPOINT")
            .expect("AZURE_ENDPOINT environment variable not set");

        Self::new(api_key, endpoint)
    }

    /// Create a client with account name using the default endpoint pattern
    pub fn with_account(api_key: String, account: &str) -> Self {
        let endpoint = DEFAULT_ENDPOINT_PATTERN.replace("{account}", account);
        Self::new(api_key, endpoint)
    }

    /// Get the API key
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Get the endpoint
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Get the internal HTTP client
    pub fn http_client(&self) -> &reqwest::Client {
        &self.client
    }
}

impl ProviderClient for Client {
    fn from_env() -> Self
    where
        Self: Sized,
    {
        Client::from_env()
    }

    fn from_val(value: ProviderValue) -> Self
    where
        Self: Sized,
    {
        match value {
            ProviderValue::Simple(api_key) => {
                let endpoint = std::env::var("AZURE_AI_FOUNDRY_ENDPOINT")
                    .expect("AZURE_AI_FOUNDRY_ENDPOINT environment variable not set");
                Client::new(api_key, endpoint)
            },
            _ => panic!("Unsupported provider value type for Azure AI Foundry"),
        }
    }
}

impl CompletionClient for Client {
    type CompletionModel = CompletionModel;

    fn completion_model(&self, model: &str) -> Self::CompletionModel {
        CompletionModel::new(self.clone(), model)
    }
}

impl VerifyClient for Client {
    async fn verify(&self) -> Result<(), VerifyError> {
        // For now, we'll just verify that we can make a basic request
        // In a real implementation, you might want to make a simple API call
        // to verify the credentials and endpoint
        Ok(())
    }
}

impl_conversion_traits!(
    AsTranscription,
    AsAudioGeneration,
    AsEmbeddings,
    AsImageGeneration for Client
);
