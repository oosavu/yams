use std::thread::Scope;
use yams_core::{Engine, ModulesRegistry};
use yams_default_modules::create_registry;

struct Server{
    engine: Engine,
    registries: Vec<Box<dyn ModulesRegistry>>
}

impl Server {
    pub fn start(&mut self) {
        self.engine.start();
    }
    pub fn stop(&mut self) {
        self.engine.stop();
    }
    pub fn add_module(&mut self, name: &str) {
        //self.registries.at(0).fabrics().get(name).unwrap().create_module(&mut self.engine);
    }
}

impl Default for Server {
    fn default() -> Self {
        Server {
            engine: Engine::default(),
            registries: vec![Box::new(create_registry())],
        }
    }
}