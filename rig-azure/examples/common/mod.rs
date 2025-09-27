use rig::tool::Tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct AdderArgs {
    /// The first number to add
    pub a: i32,
    /// The second number to add
    pub b: i32,
}

#[derive(Debug, thiserror::Error)]
#[error("Math error")]
pub struct MathError;

pub struct Adder;

impl Tool for Adder {
    const NAME: &'static str = "add";

    type Args = AdderArgs;
    type Output = i32;
    type Error = MathError;

    async fn definition(&self, _prompt: String) -> rig::completion::ToolDefinition {
        rig::completion::ToolDefinition {
            name: "add".to_string(),
            description: "Add two numbers together".to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(AdderArgs)).expect("Tool definition should serialize to JSON"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(args.a + args.b)
    }
}
