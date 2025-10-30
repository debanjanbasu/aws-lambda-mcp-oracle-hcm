use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Employee {
    #[schemars(description = "Unique Westpac Employee ID, e.g. M061230")]
    pub wbc_employee_id: String,
    #[schemars(description = "Unique PersonID in Oracle HCM, e.g. 300000578701661")]
    pub hcm_person_id: Option<String>,
}