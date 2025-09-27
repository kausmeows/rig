# rig-azure

Azure OpenAI Service integration for the Rig framework.

## Features

- Azure OpenAI Service chat completion models
- Support for deployed GPT models (gpt-4o, gpt-4o-mini, gpt-35-turbo, etc.)
- Compatible with Rig's agent system
- Environment-based configuration
- Full tool and context support

## Usage

```rust
use rig::prelude::*;
use rig_azure::Client;

// Create Azure client from environment
let client = Client::from_env();

// Create agent with your deployed model name
let agent = client
    .agent("gpt-4o-mini")  // Use your Azure OpenAI deployment name
    .preamble("You are a helpful assistant.")
    .build();

// Prompt the agent
let response = agent.prompt("Hello!").await?;
println!("{}", response);
```

## Configuration

Create a `.env` file in your project root:

```env
# Your Azure OpenAI Service API key
AZURE_API_KEY=your-api-key-here

# Your Azure OpenAI Service endpoint 
AZURE_ENDPOINT=https://your-resource.openai.azure.com
```

**Note**: The model name should match your **deployment name** in Azure OpenAI Service, not the base model name.
