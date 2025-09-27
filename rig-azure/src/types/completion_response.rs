use rig::completion::{CompletionResponse, Usage, AssistantContent};
use rig::message::Text;
use rig::one_or_many::OneOrMany;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<AzureChoice>,
    pub usage: AzureUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureChoice {
    pub index: usize,
    pub message: AzureResponseMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureResponseMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureUsage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

impl TryFrom<AzureCompletionResponse> for CompletionResponse<AzureCompletionResponse> {
    type Error = String;

    fn try_from(response: AzureCompletionResponse) -> Result<Self, Self::Error> {
        let choice = response.choices.first()
            .ok_or("No choices in response")?;

        let usage = Usage {
            input_tokens: response.usage.prompt_tokens,
            output_tokens: response.usage.completion_tokens,
            total_tokens: response.usage.total_tokens,
        };

        // Convert the string content to AssistantContent
        let assistant_content = AssistantContent::Text(Text {
            text: choice.message.content.clone(),
        });

        Ok(CompletionResponse {
            choice: OneOrMany::one(assistant_content),
            usage,
            raw_response: response,
        })
    }
}
