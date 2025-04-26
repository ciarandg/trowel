#![allow(dead_code)] // Lots of unused properties that I still want to strictly require since they're part of the TF spec

use std::collections::HashMap;

use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlan {
    pub format_version: String,
    pub terraform_version: String,
    pub planned_values: TfPlanPlannedValues,
    pub resource_drift: Option<Vec<Value>>,
    pub resource_changes: Option<Vec<TfPlanResourceChange>>,
    pub prior_state: Option<TfPlanPriorState>,
    pub configuration: TfPlanConfiguration,
    pub relevant_attributes: Option<Vec<TfPlanRelevantAttribute>>,
    pub checks: Option<Vec<TfPlanCheck>>,
    pub timestamp: String,
    pub applyable: Option<bool>,
    pub complete: Option<bool>,
    pub errored: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanPlannedValues {
    pub root_module: TfPlanPlannedValuesRootModule,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanPlannedValuesRootModule {
    pub resources: Option<Vec<TfPlanPlannedValuesModuleResource>>,
    pub child_modules: Option<Vec<TfPlanPlannedValuesChildModule>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanPlannedValuesModuleResource {
    pub address: String,
    pub mode: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub name: String,
    pub index: Option<String>,
    pub provider_name: String,
    pub schema_version: u8,
    pub values: HashMap<String, Value>,
    pub sensitive_values: HashMap<String, Value>,
    pub depends_on: Option<Vec<String>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanPlannedValuesChildModule {
    pub address: String,
    pub resources: Option<Vec<TfPlanPlannedValuesModuleResource>>,
    pub child_modules: Option<Vec<TfPlanPlannedValuesChildModule>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanResourceChange {
    pub address: String,
    pub mode: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub name: String,
    pub index: Option<String>,
    pub provider_name: String,
    pub change: TfPlanResourceChangeChange,
    pub action_reason: Option<String>,
    pub module_address: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanResourceChangeChange {
    pub actions: Vec<String>,
    pub before: Option<HashMap<String, Value>>,
    pub after: Option<HashMap<String, Value>>,
    pub after_unknown: HashMap<String, Value>,
    pub before_sensitive: SensitiveValues,
    pub after_sensitive: SensitiveValues,
    pub importing: Option<Value>,
    pub replace_paths: Option<Vec<Vec<String>>>,
}

pub type SensitiveValuesInner = Option<HashMap<String, Value>>;
pub struct SensitiveValues(SensitiveValuesInner);

impl SensitiveValues {
    pub fn new(inner: SensitiveValuesInner) -> Self {
        Self(inner)
    }

    pub fn inner(&self) -> &SensitiveValuesInner {
        &self.0
    }
}

impl<'de> Deserialize<'de> for SensitiveValues {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SensitiveValuesVisitor;

        impl<'de> serde::de::Visitor<'de> for SensitiveValuesVisitor {
            type Value = SensitiveValues;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("False or a map")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v == false {
                    Ok(SensitiveValues(None))
                } else {
                    Err(E::custom("Expected false or a map"))
                }
            }

            fn visit_map<A>(self, visitor: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let map = Map::deserialize(serde::de::value::MapAccessDeserializer::new(visitor))?;
                Ok(SensitiveValues(Some(map.into_iter().collect())))
            }
        }

        deserializer.deserialize_any(SensitiveValuesVisitor)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanPriorState {
    pub format_version: String,
    pub terraform_version: String,
    pub values: TfPlanPriorStateValues,
}

type TfPlanPriorStateValues = TfPlanPlannedValues;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanConfiguration {
    pub provider_config: Option<HashMap<String, TfPlanConfigurationProviderConfig>>,
    pub root_module: Value,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanConfigurationProviderConfig {
    pub name: String,
    pub full_name: String,
    pub version_constraint: Option<String>,
    pub expressions: Option<HashMap<String, Value>>,
    pub module_address: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanConfigurationRootModule {
    pub resources: Vec<Value>,                // TODO
    pub module_calls: HashMap<String, Value>, // TODO
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanRelevantAttribute {
    pub resource: String,
    pub attribute: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanCheck {
    pub address: TfPlanCheckAddress,
    pub status: String,
    pub instances: Vec<TfPlanCheckInstance>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanCheckAddress {
    pub kind: String,
    pub module: String,
    pub name: String,
    pub to_display: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanCheckInstance {
    pub address: TfPlanCheckInstanceAddress,
    pub status: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TfPlanCheckInstanceAddress {
    pub module: String,
    pub to_display: String,
}
