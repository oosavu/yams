use yams_core::{Engine, ModulesRegistry};
use yams_default_modules::create_registry;


pub struct Server{
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

    // pub fn command(&mut self, command: &[u8]) -> Result<Vec<u8>, yams_proto::Error> {
    //     // let fb = yams_proto::root_as_message(command);
    //     //
    //     // let mut builder = flatbuffers::FlatBufferBuilder::new();
    //     // builder
    // }
}

impl Default for Server {
    fn default() -> Self {
        Server {
            engine: Engine::default(),
            registries: vec![Box::new(create_registry())],
        }
    }
}