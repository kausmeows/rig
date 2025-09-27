# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-12-28

### Added
- Initial release of rig-azure
- Azure AI Foundry client implementation
- Support for chat completion models (GPT-4o, GPT-4o-mini, GPT-35-turbo)
- Authentication via API key and endpoint configuration
- Integration with Rig's agent system
- Environment variable configuration support
- Example implementation demonstrating usage with tools, context, and loaders
- Full compatibility with Rig's completion traits and interfaces

### Features
- **Client**: Azure AI Foundry REST API client with configurable endpoints
- **Completion Models**: Support for Azure-hosted OpenAI models
- **Agent Integration**: Compatible with Rig's agent builder and tool system
- **Error Handling**: Comprehensive error types for Azure API responses
- **Configuration**: Environment-based setup (AZURE_API_KEY, AZURE_ENDPOINT)

### Examples
- `agent_with_azure.rs`: Comprehensive example showing basic agents, tool usage, context injection, and document loading
