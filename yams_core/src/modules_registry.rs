use std::collections::HashMap;
use crate::ModuleFabric;

pub trait ModulesRegistry {
    fn fabrics(&self) -> &HashMap<String, Box<dyn ModuleFabric>>;
}
