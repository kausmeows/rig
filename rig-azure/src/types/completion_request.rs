use rig::completion::{CompletionRequest, Message};
use rig::message::{UserContent, ToolResultContent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureCompletionRequest {
    pub messages: Vec<AzureMessage>,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureMessage {
    pub role: String,
    pub content: String,
}

impl From<CompletionRequest> for AzureCompletionRequest {
    fn from(request: CompletionRequest) -> Self {
        // Convert the completion request to Azure format
        let mut messages = Vec::new();

        // Add system message if present
        if let Some(preamble) = request.preamble {
            messages.push(AzureMessage {
                role: "system".to_string(),
                content: preamble,
            });
        }

        // Convert chat history  
        let chat_messages: Vec<_> = request.chat_history.into_iter().collect();

        for message in chat_messages {
            let (role, content) = match message {
                Message::User { content, .. } => {
                let text: Vec<String> = content.into_iter()
                    .map(format_user_content)
                    .collect();
                let text = text.join("\n");
                    ("user".to_string(), text)
                },
                Message::Assistant { content, .. } => {
                let text: Vec<String> = content.into_iter()
                    .map(format_assistant_content)
                    .collect();
                let text = text.join("\n");
                    ("assistant".to_string(), text)
                },
            };

            messages.push(AzureMessage {
                role,
                content,
            });
        }

        Self {
            messages,
            model: "gpt-4o".to_string(), // This will be overridden by the CompletionModel
            max_tokens: request.max_tokens,
            temperature: request.temperature,
            top_p: None, // Azure AI Foundry might support this, but not in CompletionRequest
            frequency_penalty: None,
            presence_penalty: None,
            stop: None,
            stream: Some(false),
        }
    }
}

fn format_user_content(content: UserContent) -> String {
    match content {
        UserContent::Text(text) => text.text,
        UserContent::Image(_) => "[Image]".to_string(),
        UserContent::Audio(_) => "[Audio]".to_string(),
        UserContent::Video(_) => "[Video]".to_string(),
        UserContent::Document(_) => "[Document]".to_string(),
        UserContent::ToolResult(result) => {
            let content_texts: Vec<String> = result.content.into_iter()
                .map(format_tool_result_content)
                .collect();
            content_texts.join("\n")
        },
    }
}

fn format_tool_result_content(content: ToolResultContent) -> String {
    match content {
        ToolResultContent::Text(text) => text.text,
        ToolResultContent::Image(_) => "[Image Result]".to_string(),
    }
}

fn format_assistant_content(content: rig::completion::AssistantContent) -> String {
    match content {
        rig::completion::AssistantContent::Text(text) => text.text,
        rig::completion::AssistantContent::ToolCall(call) => format!("[Tool Call: {}]", call.function.name),
        rig::completion::AssistantContent::Reasoning(reasoning) => reasoning.reasoning.join("\n"),
    }
}
