use std::{fs, path::Path};

use quick_xml::{de::from_str, DeError};
use serde::Deserialize;

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
    #[serde(rename = "@valueReference")]
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

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ScalarVariable {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@valueReference")]
    pub value_reference: usize,
    #[serde(rename = "@description")]
    pub description: String,
    #[serde(rename = "@causality")]
    pub causality: String,
    #[serde(rename = "@variability")]
    pub variability: String,
    #[serde(rename = "@initial")]
    pub initial: String,
    // #[serde(rename = "@canHandleMultipleSetPerTimeInstant")]
    // pub can_handle_multiple_set_per_time_instant: bool,
    pub real: Option<Real>,
    pub integer: Option<Integer>,
    pub boolean: Option<Boolean>,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ModelVariables {
    pub scalar_variable: Vec<ScalarVariable>,
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
        let text = fs::read_to_string(&path).unwrap();
        from_str(&text)
    }
}

#[test]
fn model_description() {
    let text = fs::read_to_string("./tests/parsing/unit-test.xml").unwrap();
    let md: FmiModelDescription = from_str(&text).unwrap();

    println!("{:?}", md.description);
    println!("{:?}", md.default_experiment);

    let text = fs::read_to_string("./tests/parsing/complex-fmi.xml").unwrap();
    let md: FmiModelDescription = from_str(&text).unwrap();

    println!("{:?}", md.description);
    println!("{:?}", md.default_experiment);
}
