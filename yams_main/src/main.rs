extern crate steel;
extern crate steel_derive;
extern crate steel_repl;

use steel::steel_vm::engine::Engine;
use steel_doc::walk_dir;
use steel_repl::run_repl;

use std::path::PathBuf;
use std::process;
use std::{error::Error, fs};

use clap::Parser;
use env_logger::Env;
use log::{debug, error};
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};
use yams_default_modules::*;
use yams_server::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// script file
    default_file: Option<PathBuf>,
}

pub fn run(clap_args: Args) -> Result<(), Box<dyn Error>> {
    let mut vm = Engine::new();
    vm.register_value("std::env::args", steel::SteelVal::ListV(vec![].into()));

    debug!("qwe clap_args: {clap_args:?}");
    match clap_args {
        Args { default_file: None } => {
            run_repl(vm)?;
            Ok(())
        }

        Args {
            default_file: Some(path),
        } => {
            let contents = fs::read_to_string(&path)?;
            let res = vm.compile_and_run_raw_program_with_path(contents.clone(), path.clone());

            if let Err(e) = res {
                vm.raise_error(e.clone());
                process::exit(1);
            }

            Ok(())
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // env_logger::Builder::from_env(Env::default().default_filter_or("yams_main=debug")).init();
    // let clap_args = Args::parse();
    // run(clap_args)?;
    Ok(())

    // let clap_args = Args::parse();
    // steel_copy::run(clap_args)?;

    // let mut server = Server::default();
    // server.exec_script("sine");

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
