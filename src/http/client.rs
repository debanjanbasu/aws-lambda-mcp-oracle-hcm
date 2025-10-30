use std::{sync::LazyLock, time::Duration};
use anyhow::{anyhow, Result};
use reqwest::{Body, Method};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use tracing::{error, info, trace};
use crate::helpers::env::{load_env_var, load_env_var_or};

static HTTP_CLIENT: LazyLock<Result<ClientWithMiddleware, String>> = LazyLock::new(|| {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {e}"))?;
    
    Ok(ClientBuilder::new(client)
        .build())
});

static HCM_BASE_URL: LazyLock<Result<String>> = 
    LazyLock::new(|| load_env_var("HCM_BASE_URL"));
static HCM_API_VERSION: LazyLock<String> = 
    LazyLock::new(|| load_env_var_or("HCM_API_VERSION", "11.13.18.05"));
static REST_FRAMEWORK_VERSION: LazyLock<String> = 
    LazyLock::new(|| load_env_var_or("REST_FRAMEWORK_VERSION", "9"));
static HCM_USERNAME: LazyLock<String> = 
    LazyLock::new(|| load_env_var_or("HCM_USERNAME", "WBC_HR_AGENT"));
static HCM_PASSWORD: LazyLock<Result<String>> = 
    LazyLock::new(|| load_env_var("HCM_PASSWORD"));

pub async fn hcm_api_call(
    path: &str,
    method: Method,
    body: Option<Body>,
    enable_framework_version: bool,
    set_timeout: Option<Duration>,
) -> Result<serde_json::Value> {
    let base = HCM_BASE_URL.as_ref()
        .map_err(|e| anyhow!("HCM_BASE_URL not set: {e}"))?;
    let api_ver = &*HCM_API_VERSION;

    let url = format!("{base}/hcmRestApi/resources/{api_ver}{path}");
    
    info!("HCM API request: {} {}", method, url);
    
    let client = HTTP_CLIENT.as_ref()
        .map_err(|e| anyhow!("HTTP client initialization failed: {e}"))?
        .clone();

    let password = HCM_PASSWORD.as_ref()
        .map_err(|e| anyhow!("HCM_PASSWORD not set: {e}"))?;
    let username = &*HCM_USERNAME;

    let mut request_builder = match method {
        Method::GET => client.get(&url),
        Method::POST => client.post(&url).body(body.unwrap_or_default()),
        _ => return Err(anyhow!("Only GET and POST methods are supported")),
    };
    
    if let Some(timeout) = set_timeout {
        request_builder = request_builder.timeout(timeout);
    }

    request_builder = request_builder.basic_auth(&username, Some(password));

    if enable_framework_version {
        let rf_version = &*REST_FRAMEWORK_VERSION;
        request_builder = request_builder.header("REST-Framework-Version", rf_version);
    }

    if method == Method::POST {
        request_builder =
            request_builder.header("Content-Type", "application/vnd.oracle.adf.action+json");
    }

    let response = request_builder.send().await?;
    let status = response.status();
    
    info!("HCM API response: {} {} - Status: {}", method, url, status);
    
    if !status.is_success() {
        let error_text = response.text().await
            .unwrap_or_else(|e| format!("Unable to read error response body: {e}"));
        
        error!("HCM API request failed with status {}: {}", status, error_text);
        return Err(anyhow!("HTTP {status}: {error_text}"));
    }
    
    let json_response = response.json::<serde_json::Value>().await?;
    
    trace!("HCM API response (JSON): {:?}", json_response);
    Ok(json_response)
}
