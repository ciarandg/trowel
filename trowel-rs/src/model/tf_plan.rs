use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlan {
    pub format_version: String,
    pub terraform_version: String,
    pub planned_values: TfPlanPlannedValues,
    pub resource_changes: Vec<TfPlanResourceChange>,
    pub prior_state: TfPlanPriorState,
    pub configuration: TfPlanConfiguration,
    pub relevant_attributes: Vec<TfPlanRelevantAttribute>,
    pub checks: Vec<TfPlanCheck>,
    pub timestamp: String,
    pub errored: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanPlannedValues {
    pub root_module: TfPlanPlannedValuesRootModule
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanPlannedValuesRootModule {
    pub resources: Vec<TfPlanPlannedValuesModuleResource>,
    pub child_modules: Vec<TfPlanPlannedValuesChildModule>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanPlannedValuesModuleResource {
    pub address: String,
    pub mode: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub name: String,
    pub provider_name: String,
    pub schema_version: u8,
    pub values: HashMap<String, Value>,
    pub sensitive_values: HashMap<String, Value>,
    pub depends_on: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanPlannedValuesChildModule {
    pub address: String,
    pub resources: Option<Vec<TfPlanPlannedValuesModuleResource>>,
    pub child_modules: Option<Vec<TfPlanPlannedValuesChildModule>>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanResourceChange {
    pub address: String,
    pub mode: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub name: String,
    pub provider_name: String,
    pub change: TfPlanResourceChangeChange,
    pub action_reason: Option<String>,
    pub module_address: Option<String>
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanResourceChangeChange {
    pub actions: Vec<String>,
    pub before: Option<HashMap<String, Value>>,
    pub after: Option<HashMap<String, Value>>,
    pub after_unknown: HashMap<String, Value>,
    pub before_sensitive: Value, // TODO this is really Either<HashMap<String, Value>, false>
    pub after_sensitive: Value, // TODO this is really Either<HashMap<String, Value>, false>
    pub replace_paths: Option<Vec<Vec<String>>>,
}

type SensitiveValues = Option<HashMap<String, Value>>;

impl TfPlanResourceChangeChange {
    pub fn process_before_sensitive(&self) -> Result<SensitiveValues, &str> {
        process_sensitive_values(&self.before_sensitive)
    }

    pub fn process_after_sensitive(&self) -> Result<SensitiveValues, &str> {
        process_sensitive_values(&self.after_sensitive)
    }
}

// TODO make a proper deserializer for this
fn process_sensitive_values(v: &Value) -> Result<SensitiveValues, &str> {
    match v {
        Value::Bool(b) => {
            if *b {
                Err("sensitive values can be boolean, but should always be false if so")
            } else {
                Ok(None)
            }
        },
        Value::Object(map) => {
            let mut result = HashMap::new();
            for (key, value) in map.iter() {
                result.insert(key.clone(), value.clone());
            }
            Ok(Some(result))
        },
        _ => Err("sensitive values should either be a false boolean or a dictionary"),
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanPriorState {
    pub format_version: String,
    pub terraform_version: String,
    pub values: TfPlanPriorStateValues,
}

type TfPlanPriorStateValues = TfPlanPlannedValues;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanConfiguration {
    pub provider_config: HashMap<String, TfPlanConfigurationProviderConfig>,
    pub root_module: Value
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanConfigurationProviderConfig {
    pub name: String,
    pub full_name: String,
    pub version_constraint: Option<String>,
    pub expressions: Option<HashMap<String, Value>>,
    pub module_address: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanConfigurationRootModule {
    pub resources: Vec<Value>, // TODO
    pub module_calls: HashMap<String, Value>, // TODO
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanRelevantAttribute {
    pub resource: String,
    pub attribute: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanCheck {
    pub address: TfPlanCheckAddress,
    pub status: String,
    pub instances: Vec<TfPlanCheckInstance>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanCheckAddress {
    pub kind: String,
    pub module: String,
    pub name: String,
    pub to_display: String,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanCheckInstance {
    pub address: TfPlanCheckInstanceAddress,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanCheckInstanceAddress {
    pub module: String,
    pub to_display: String,
}