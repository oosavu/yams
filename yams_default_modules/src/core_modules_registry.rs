use std::sync::{Arc, Mutex};
use yams_core::{Module, ModuleArc, ModuleInfo, ModulesRegistry};
use crate::sine::ModuleSine;
use crate::audio_io::ModuleIO;


pub struct Scope{
    modules_info: Vec<ModuleInfo>
}

impl ModulesRegistry for Scope {
    fn modules(&self) -> &Vec<ModuleInfo> {
        &self.modules_info
    }

    fn create_module(name: &str) -> Option<ModuleArc> {
        match name {
            "audio_io" => Some(Arc::new(Mutex::new(ModuleIO::default())) as ModuleArc),
            "sine" => Some(Arc::new(Mutex::new(ModuleSine::default())) as ModuleArc),
            _ => None,
        }
    }
}