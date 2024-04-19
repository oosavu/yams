use crate::cable::*;
use crate::module::*;
pub use std::sync::atomic::{AtomicBool, Ordering};
pub use std::sync::{Arc, Condvar, Mutex};
pub use std::thread;
use std::time::{Duration, SystemTime};

// low level structure for working with raw pointers
pub struct RealTimeCore {
    pub modules_pointers: Vec<ModulePointer>,
    pub default_module: ModulePointer,
    pub cable_core: Vec<Cable>,
    pub sample_rate: i64,
    pub current_time: SystemTime,
    #[allow(unused)]
    alive: Arc<AtomicBool>,
    #[allow(unused)]
    is_fallback_active: Arc<(Mutex<bool>, Condvar)>,
}

unsafe impl Send for RealTimeCore {}
unsafe impl Sync for RealTimeCore {}

impl RealTimeCore {
    pub fn compute_frame(&mut self, time_frame: usize) {
        // TODO does it vectorized automativally?
        unsafe {
            for _ in 0..time_frame {
                for m in self.modules_pointers.iter_mut() {
                    let qwe = &mut *m.unwrap().as_mut();
                    qwe.process();
                }
                for c in self.cable_core.iter_mut() {
                    let input_m = &mut *c.input_module_p.unwrap().as_mut();
                    let output_m = &mut *c.output_module_p.unwrap().as_mut();
                    output_m.inputs()[c.output_port].value = input_m.outputs()[c.input_port].value;
                }
            }
        }
    }
}

pub type RealTimeCoreArc = Arc<Mutex<RealTimeCore>>;
