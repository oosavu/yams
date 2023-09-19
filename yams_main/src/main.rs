extern crate yams_core;
use yams_core::*;
use std::{thread, time::Duration};
use std::sync::{Arc, Mutex};

fn main() {
    let mut e = yams_core::Engine::default();

    let mut ms: ModuleArc = Arc::new(Mutex::new(ModuleSine::default()));
    let mut ma: ModuleArc = Arc::new(Mutex::new(ModuleO::default()));

    e.add_module(&mut ms);
    e.add_module(&mut ma);
    e.add_cable(Cable::new_cable(&mut ms, &mut ma, 0, 0));

    e.start();
    thread::sleep(Duration::from_millis(3000));
    e.stop();
}
