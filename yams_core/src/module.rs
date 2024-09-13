use crate::synth_core::RealTimeCoreArc;
use crate::{AudioPort, Parameter};
use crate::ModuleInfo;
use std::ptr::NonNull;
use std::sync::{Arc, Mutex};

pub type ModuleArc = Arc<Mutex<dyn Module>>;
pub type ModulePointer = Option<NonNull<dyn Module>>;
pub type DriverCallback = Arc<Mutex<Box<dyn Fn() + Send + Sync>>>;

pub trait AudioDriver {
    fn recommended_framerate(&self) -> cpal::SampleRate;
    fn start_process(&mut self, rt_core: RealTimeCoreArc);
    fn stop(&mut self);
}

pub type AudioDriverArc = Arc<Mutex<dyn AudioDriver>>;

pub trait Module {
    fn set_framerate(&mut self, framerate: f64);
    fn process(&mut self);
    fn inputs(&mut self) -> &mut Vec<AudioPort>;
    fn outputs(&mut self) -> &mut Vec<AudioPort>;
    fn parameters(&mut self) -> &mut Vec<Parameter>;

    fn audio_driver(&self) -> Option<AudioDriverArc>;
}

// extract unsafe fat pointer
pub fn extract_pointer(module: &ModuleArc) -> ModulePointer {
    return unsafe {
        let asd: &Mutex<dyn Module> = module;
        let qwe: *mut dyn Module = &mut *asd.lock().unwrap() as *mut dyn Module;
        Some(NonNull::new_unchecked(qwe))
    };
}

pub trait ModuleFabric{
    fn info(&self) -> &ModuleInfo;
    fn create(&self) -> ModuleArc;
}
