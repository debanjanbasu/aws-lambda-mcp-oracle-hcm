use serde_json::{json, Value};
use lambda_runtime::Diagnostic;
use tracing::debug;

pub async fn function_handler(event: lambda_runtime::LambdaEvent<Value>) -> Result<Value, Diagnostic> {
    let (_event, _context) = event.into_parts();
    
    // Get the toolname from the context
    let tool_name = "unknown"; // Temporary placeholder

    debug!(tool_name = ?tool_name, "Tool name from context");
    
    // Generate response and serialize to JSON
    // let response = create_greeting(name)?;
    Ok(json!({ "message": "Lambda response" }))
}