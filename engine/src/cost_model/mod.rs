use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct OperationCost {
    pub time: String,
    pub space: String,
    #[serde(default)]
    pub amortized: bool,
    #[serde(default)]
    pub expected: bool,
    pub worst: Option<String>,
    pub hidden_cost: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContainerCosts {
    #[serde(flatten)]
    pub operations: HashMap<String, OperationCost>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CostModelData {
    pub containers: HashMap<String, ContainerCosts>,
}

pub struct CostModel {
    pub cpp_costs: CostModelData,
    pub java_costs: CostModelData,
}

impl Default for CostModel {
    fn default() -> Self {
        let cpp_str = include_str!("cpp_costs.toml");
        let java_str = include_str!("java_costs.toml");

        let cpp_costs: CostModelData =
            toml::from_str(cpp_str).expect("Failed to parse cpp_costs.toml");
        let java_costs: CostModelData =
            toml::from_str(java_str).expect("Failed to parse java_costs.toml");

        Self {
            cpp_costs,
            java_costs,
        }
    }
}

impl CostModel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lookup_cpp_cost(&self, container: &str, operation: &str) -> Option<&OperationCost> {
        self.cpp_costs
            .containers
            .get(container)?
            .operations
            .get(operation)
    }

    pub fn lookup_java_cost(&self, container: &str, operation: &str) -> Option<&OperationCost> {
        self.java_costs
            .containers
            .get(container)?
            .operations
            .get(operation)
    }
}
