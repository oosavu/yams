use std::collections::HashMap;

use yams_core::{ModuleFabric, ModulesRegistry};

use crate::audio_io::ModuleIOFabric;
use crate::sine::ModuleSineFabric;

pub struct CoresRegistry {
    fabrics: HashMap<String, Box<dyn ModuleFabric>>,
}

impl ModulesRegistry for CoresRegistry {
    fn fabrics(&self) -> &HashMap<String, Box<dyn ModuleFabric>> {
        return &self.fabrics;
    }
}

pub fn create_registry() -> CoresRegistry {
    CoresRegistry {
        fabrics: HashMap::from([
            (
                "sine".to_string(),
                Box::new(ModuleSineFabric::default()) as Box<dyn ModuleFabric>,
            ),
            (
                "audio_io".to_string(),
                Box::new(ModuleIOFabric::default()) as Box<dyn ModuleFabric>,
            ),
        ]),
    }
}
