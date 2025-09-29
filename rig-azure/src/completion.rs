use crate::client::Client;
use crate::types::{
    completion_request::AzureCompletionRequest,
    completion_response::AzureCompletionResponse,
    error::{AzureApiError, AzureError},
};
use rig::completion::{self, CompletionError, CompletionRequest};
use rig::streaming::StreamingCompletionResponse;

// Common Azure AI Foundry model identifiers
pub const GPT_4O: &str = "gpt-4o";
pub const GPT_4O_MINI: &str = "gpt-4o-mini";
pub const GPT_35_TURBO: &str = "gpt-35-turbo";
pub const PHI_4: &str = "Phi-4";

#[derive(Clone, Debug)]
pub struct CompletionModel {
    client: Client,
    model: String,
}

impl CompletionModel {
    pub fn new(client: Client, model: &str) -> Self {
        Self {
            client,
            model: model.to_string(),
        }
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Build the completion endpoint URL for Azure AI Foundry
    /// Pattern: https://{resource}.services.ai.azure.com/models/chat/completions?api-version=2024-05-01-preview
    fn completion_url(&self) -> String {
        let base_endpoint = self.client.endpoint().trim_end_matches("/");
        format!("{}/chat/completions?api-version=2024-05-01-preview", base_endpoint)
    }

    async fn send_completion_request(
        &self,
        mut request: AzureCompletionRequest,
    ) -> Result<AzureCompletionResponse, AzureError> {
        // Set the model name in the request
        request.model = self.model.clone();

        let url = self.completion_url();
        
        // Optional debug logging (enable with RUST_LOG=debug)
        tracing::debug!("Making Azure AI Foundry request to: {}", url);
        tracing::debug!("Model: {}, Endpoint: {}", self.model, self.client.endpoint());
        
        let response = self
            .client
            .http_client()
            .post(&url)
            .header("api-key", self.client.api_key())  // Azure AI Foundry uses 'api-key' header
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        let headers = response.headers().clone();
        
        if status.is_success() {
            tracing::info!("✅ Request successful!");
            let completion_response: AzureCompletionResponse = response.json().await?;
            Ok(completion_response)
        } else {
            let status_code = status.as_u16();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            
            // DEBUG: Log the error response
            tracing::error!("❌ Request failed:");
            tracing::error!("  Status: {}", status_code);
            tracing::error!("  Response body: {}", error_text);
            tracing::error!("  Response headers: {:?}", headers);
            
            // Try to parse as Azure API error format
            if let Ok(api_error) = serde_json::from_str::<AzureApiError>(&error_text) {
                Err(AzureError::ApiError {
                    message: api_error.error.message,
                    status: status_code,
                })
            } else {
                Err(AzureError::ApiError {
                    message: error_text,
                    status: status_code,
                })
            }
        }
    }
}

impl completion::CompletionModel for CompletionModel {
    type Response = AzureCompletionResponse;
    type StreamingResponse = (); // Streaming not implemented yet

    async fn completion(
        &self,
        completion_request: CompletionRequest,
    ) -> Result<completion::CompletionResponse<AzureCompletionResponse>, CompletionError> {
        let azure_request = AzureCompletionRequest::from(completion_request);
        let response = self.send_completion_request(azure_request).await?;
        response.try_into().map_err(|e: String| CompletionError::ProviderError(e))
    }

    async fn stream(
        &self,
        _request: CompletionRequest,
    ) -> Result<StreamingCompletionResponse<Self::StreamingResponse>, CompletionError> {
        Err(CompletionError::RequestError(
            "Streaming not implemented yet for Azure AI Foundry".to_string().into(),
        ))
    }
}
