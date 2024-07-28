use yams_core::{Engine, ModulesRegistry};
use yams_default_modules::create_registry;
use steel::steel_vm::*;
use steel::SteelVal;

pub struct Server{
    engine: Engine,
    registries: Vec<Box<dyn ModulesRegistry>>,
    steel_engine: engine::Engine
}

impl Server {
    pub fn start(&mut self) {
        self.engine.start();
    }
    pub fn stop(&mut self) {
        self.engine.stop();
    }
    pub fn exec_script(&mut self, script: &str) -> String {
        let mut steel_engine = engine::Engine::new();
        let answer = steel_engine.run( script.to_string());
        match answer {
            Ok(x) => "".to_string(),
            Err(x) => "".to_string()
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Server {
            engine: Engine::default(),
            registries: vec![Box::new(create_registry())],
            steel_engine: engine::Engine::new(),
        }
    }
}