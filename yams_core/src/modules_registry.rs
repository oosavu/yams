use crate::module_info::ModuleInfo;
use crate::ModuleArc;

pub trait ModulesRegistry{
    fn modules(&self) -> &Vec<ModuleInfo>;
    fn create_module(name: &str) -> Option<ModuleArc>;
}