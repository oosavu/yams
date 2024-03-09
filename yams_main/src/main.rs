extern crate yams_core;
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};
use yams_core::*;

fn main() {
    let mut e = yams_core::Engine::default();

    #[allow(clippy::arc_with_non_send_sync)]
    let mut ms: ModuleArc = Arc::new(Mutex::new(ModuleSine::default()));
    #[allow(clippy::arc_with_non_send_sync)]
    let mut ma: ModuleArc = Arc::new(Mutex::new(ModuleO::default()));

    e.add_module(&mut ms);
    e.add_module(&mut ma);
    e.add_cable(Cable::new_cable(&mut ms, &mut ma, 0, 0));

    e.start();
    thread::sleep(Duration::from_millis(3000));
    e.stop();
}
