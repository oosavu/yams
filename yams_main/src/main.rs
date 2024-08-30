mod steel_copy;

use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};
use yams_default_modules::*;
use yams_server::*;

use std::error::Error;

use clap::Parser;
use steel_copy::Args;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let clap_args = Args::parse();
    steel_copy::run(clap_args)?;
    Ok(())
    // let clap_args = Args::parse();
    // steel_copy::run(clap_args)?;

   // let mut server = Server::default();
   // server.exec_script("sine");

    // server
    // let mut e = Engine::default();
    //
    // #[allow(clippy::arc_with_non_send_sync)]
    //     let mut ms1: ModuleArc = Arc::new(Mutex::new(ModuleSine::default()));
    // ms1.lock().unwrap().inputs()[0].value[0] = -12.0;
    // #[allow(clippy::arc_with_non_send_sync)]
    //     let mut ms2: ModuleArc = Arc::new(Mutex::new(ModuleSine::default()));
    // #[allow(clippy::arc_with_non_send_sync)]
    //     let mut ma: ModuleArc = Arc::new(Mutex::new(ModuleIO::default()));
    //
    // e.add_module(&mut ms1);
    // e.add_module(&mut ms2);
    // e.add_module(&mut ma);
    // e.add_cable(Cable::new_cable(&mut ms2, &mut ma, 0, 0));
    //
    //
    // e.start();
    // thread::sleep(Duration::from_millis(5000));
    // e.add_cable(Cable::new_cable(&mut ms1, &mut ms2, 0, 0));
    // thread::sleep(Duration::from_millis(5000));
    // e.stop();
}
