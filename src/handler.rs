use serde_json::{json, Value};
use lambda_runtime::Diagnostic;
use tracing::debug;
use anyhow::anyhow;

pub async fn function_handler(event: lambda_runtime::LambdaEvent<Value>) -> Result<Value, Diagnostic> {
    let (event, context) = event.into_parts();
    
    let client_context = context.client_context.ok_or_else(|| anyhow!("Missing client context"))?;
    let tool_name = client_context.custom.get("bedrockAgentCoreToolName").ok_or_else(|| anyhow!("Missing bedrockAgentCoreToolName in custom context"))?;

    debug!(tool_name = ?tool_name, "Tool name from context");
    debug!(event = ?event, "Event received by Lambda");
    
    // Generate response and serialize to JSON
    // let response = create_greeting(name)?;
    Ok(json!({ "message": "Lambda response" }))
}