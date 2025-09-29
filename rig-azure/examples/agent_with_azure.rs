use rig::client::CompletionClient;
use rig::{agent::AgentBuilder, completion::Prompt};
use rig_azure::{client::Client, completion::PHI_4};
use tracing::info;

/// Runs 4 agents based on Azure AI Foundry (derived from the agent_with_bedrock example)
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Load environment variables from .env file if it exists
    dotenvy::dotenv().ok();
    
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    info!("Running basic agent");
    basic().await?;

    info!("\n\nAgent ran successfully");
    Ok(())
}

fn client() -> Client {
    Client::from_env()
}

async fn partial_agent() -> AgentBuilder<rig_azure::completion::CompletionModel> {
    let client = client();
    client.agent(PHI_4)
}

/// Create an Azure AI Foundry agent with a system prompt
async fn basic() -> Result<(), anyhow::Error> {
    let agent = partial_agent()
        .await
        .preamble("Answer with a short paragraph")
        .build();

    let response = agent.prompt("Describe solar system").await?;
    info!("{}", response);

    Ok(())
}