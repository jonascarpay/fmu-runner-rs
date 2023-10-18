use std::{
    collections::HashMap,
    fs,
    hash::{Hash, Hasher},
    path::Path,
};

use quick_xml::{de::from_str, DeError};
use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
#[allow(non_snake_case)]
pub struct BaseUnit {
    #[serde(rename = "@kg")]
    pub kg: Option<i32>,
    #[serde(rename = "@m")]
    pub m: Option<i32>,
    #[serde(rename = "@s")]
    pub s: Option<i32>,
    #[serde(rename = "@A")]
    pub A: Option<i32>,
    #[serde(rename = "@K")]
    pub K: Option<i32>,
    #[serde(rename = "@mol")]
    pub mol: Option<i32>,
    #[serde(rename = "@cd")]
    pub cd: Option<i32>,
    #[serde(rename = "@rad")]
    pub rad: Option<i32>,
    #[serde(rename = "@factor")]
    pub factor: Option<f64>,
    #[serde(rename = "@offset")]
    pub offset: Option<f64>,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct DisplayUnit {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@factor")]
    pub factor: Option<f64>,
    #[serde(rename = "@offset")]
    pub offset: Option<f64>,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Unit {
    #[serde(rename = "@name")]
    pub name: String,
    // #[serde(rename = "@BaseUnit")]
    pub base_unit: Option<BaseUnit>,
    pub display_unit: Vec<DisplayUnit>,
    #[serde(rename = "@offset")]
    pub offset: f64,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct UnitDefinitions {
    pub unit: Vec<Unit>,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Real {
    #[serde(rename = "@declaredType")]
    declared_type: Option<String>,
    #[serde(rename = "@start")]
    start: Option<f64>,
    #[serde(rename = "@derivative")]
    derivative: Option<usize>,
    #[serde(rename = "@reinit")]
    reinit: Option<bool>,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Boolean {
    #[serde(rename = "@declaredType")]
    declared_type: Option<String>,
    #[serde(rename = "@start")]
    start: Option<bool>,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Integer {
    #[serde(rename = "@declaredType")]
    declared_type: Option<String>,
    #[serde(rename = "@start")]
    start: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SignalType {
    Real(Real),
    Integer(Integer),
    Boolean(Boolean),
    String,
    Enumeration,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Causality {
    Parameter,
    CalculatedParameter,
    Input,
    Output,
    Local,
    Independent,
}

impl Default for Causality {
    fn default() -> Self {
        Causality::Local
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Variability {
    Constant,
    Fixed,
    Tunable,
    Discrete,
    Continuous,
}

impl Default for Variability {
    fn default() -> Self {
        Variability::Continuous
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Initial {
    Exact,
    Approx,
    Calculated,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScalarVariable {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@valueReference")]
    pub value_reference: ::std::os::raw::c_uint,
    #[serde(default, rename = "@description")]
    pub description: String,
    #[serde(default, rename = "@causality")]
    pub causality: Causality,
    #[serde(default, rename = "@variability")]
    pub variability: Variability,
    #[serde(rename = "@initial")]
    pub initial: Option<Initial>,
    #[serde(rename = "@canHandleMultipleSetPerTimeInstant")]
    pub can_handle_multiple_set_per_time_instant: Option<bool>,
    pub annotations: Option<()>,
    #[serde(rename = "$value")]
    pub signal_type: SignalType,
}

impl PartialEq for ScalarVariable {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for ScalarVariable {}

impl Hash for ScalarVariable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

fn deserialize_to_map<'de, D>(deserializer: D) -> Result<HashMap<String, ScalarVariable>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Vec::<ScalarVariable>::deserialize(deserializer)?;
    let mut map = HashMap::new();
    for item in v {
        map.insert(item.name.clone(), item);
    }
    Ok(map)
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ModelVariables {
    #[serde(deserialize_with = "deserialize_to_map")]
    pub scalar_variable: HashMap<String, ScalarVariable>,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct FMIFile {
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct FMISourceFiles {
    #[serde(rename = "@name")]
    pub file: Vec<FMIFile>,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Category {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@description")]
    pub description: String,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct LogCategories {
    pub category: Vec<Category>,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ModelExchange {
    pub source_files: FMISourceFiles,
    #[serde(rename = "@modelIdentifier")]
    pub model_identifier: String,
    #[serde(rename = "@needsExecutionTool")]
    pub needs_execution_tool: bool,
    #[serde(rename = "@completedIntegratorStepNotNeeded")]
    pub completed_integrator_step_not_needed: bool,
    #[serde(rename = "@canBeInstantiatedOnlyOncePerProcess")]
    pub can_be_instantiated_only_once_per_process: bool,
    #[serde(rename = "@canNotUseMemoryManagementFunctions")]
    pub can_not_use_memory_management_functions: bool,
    #[serde(rename = "@canGetAndSetFMUstate")]
    pub can_get_and_set_fmustate: bool,
    #[serde(rename = "@canSerializeFMUstate")]
    pub can_serialize_fmustate: bool,
    #[serde(rename = "@providesDirectionalDerivative")]
    pub provides_directional_derivative: bool,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct CoSimulation {
    pub source_files: FMISourceFiles,
    #[serde(rename = "@modelIdentifier")]
    pub model_identifier: String,
    #[serde(rename = "@needsExecutionTool")]
    pub needs_execution_tool: bool,
    #[serde(rename = "@canHandleVariableCommunicationStepSize")]
    pub can_handle_variable_communication_step_size: bool,
    #[serde(rename = "@canInterpolateInputs")]
    pub can_interpolate_inputs: bool,
    #[serde(rename = "@maxOutputDerivativeOrder")]
    pub max_output_derivative_order: bool,
    #[serde(rename = "@canRunAsynchronuously")]
    pub can_run_asynchronuously: bool,
    #[serde(rename = "@canBeInstantiatedOnlyOncePerProcess")]
    pub can_be_instantiated_only_once_per_process: bool,
    #[serde(rename = "@canNotUseMemoryManagementFunctions")]
    pub can_not_use_memory_management_functions: bool,
    #[serde(rename = "@canGetAndSetFMUstate")]
    pub can_get_and_set_fmustate: bool,
    #[serde(rename = "@canSerializeFMUstate")]
    pub can_serialize_fmustate: bool,
    #[serde(rename = "@providesDirectionalDerivative")]
    pub provides_directional_derivative: bool,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct DefaultExperiment {
    #[serde(rename = "@startTime")]
    pub start_time: f64,
    #[serde(rename = "@stopTime")]
    pub stop_time: f64,
    #[serde(rename = "@tolerance")]
    pub tolerance: f64,
    #[serde(rename = "@stepSize")]
    pub step_size: Option<f64>,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct FmiModelDescription {
    pub co_simulation: Option<CoSimulation>,
    pub model_exchange: Option<ModelExchange>,
    pub model_variables: ModelVariables,
    pub unit_definitions: Option<UnitDefinitions>,
    pub log_categories: Option<LogCategories>,
    pub default_experiment: Option<DefaultExperiment>,
    // TypeDefinitions
    // VendorAnnotations
    // ModelStructure
    #[serde(rename = "@fmiVersion")]
    pub fmi_version: String,
    #[serde(rename = "@modelName")]
    pub model_name: String,
    #[serde(rename = "@guid")]
    pub guid: String,
    #[serde(rename = "@description")]
    pub description: String,
    #[serde(rename = "@author")]
    pub author: String,
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "@copyright")]
    pub copyright: String,
    #[serde(rename = "@license")]
    pub license: String,
    #[serde(rename = "@generationTool")]
    pub generation_tool: String,
    #[serde(rename = "@generationDateAndTime")]
    pub generation_date_and_time: String,
    #[serde(rename = "@variableNamingConvention")]
    pub variable_naming_convention: String,
    #[serde(rename = "@numberOfEventIndicators")]
    pub number_of_event_indicators: String,
}

impl FmiModelDescription {
    pub fn new(path: &Path) -> Result<Self, DeError> {
        let text = fs::read_to_string(path).unwrap();
        from_str(&text)
    }
}

// test module
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("./tests/parsing/unit-test.xml")]
    #[case("./tests/parsing/complex-fmi.xml")]
    #[case("./tests/parsing/bouncing-ball.xml")]
    fn test_parsing_model_description(#[case] xml: &str) {
        let text = fs::read_to_string(xml).unwrap();
        let md: FmiModelDescription = from_str(&text).unwrap();

        println!("{:?}", md.description);
        println!("{:?}", md.default_experiment);
        println!("{:?}", md.model_variables);
        println!("{:?}", md.model_variables.scalar_variable);
    }
}
