use anyhow::{anyhow, Result};
use reqwest::Method;
use crate::models::employee::Employee;
use crate::http::client::hcm_api_call;

pub async fn passthrough_cache_person_id(employee: &Employee) -> Result<Employee> {
    // Build API query - convert to uppercase as HCM stores IDs in uppercase
    // Limit to 1 result since employee IDs are unique
    let path = format!(
        "/publicWorkers?q=assignments.WorkerNumber='{}'&onlyData=true&limit=1",
        employee.wbc_employee_id
    );
    let response_json = hcm_api_call(&path, Method::GET, None, true, None).await?;

    // Extract PersonId from the response
    let hcm_person_id = response_json["items"]
        .as_array()
        .ok_or_else(|| anyhow!("Invalid response format: 'items' is not an array"))?
        .first()
        .ok_or_else(|| anyhow!("No worker found for employee ID: {}", employee.wbc_employee_id))?
        .get("PersonId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("PersonId not found in worker data"))?
        .to_string();

    Ok(Employee {
        wbc_employee_id: employee.wbc_employee_id.clone(),
        hcm_person_id: Some(hcm_person_id),
    })
}