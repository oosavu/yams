use crate::{AudioPort};
use crate::synth_core::RealTimeCoreArc;
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
    fn set_framerate(&mut self, framerate: i64);
    fn process(&mut self);
    fn inputs(&mut self) -> &mut Vec<AudioPort>;
    fn outputs(&mut self) -> &mut Vec<AudioPort>;
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

pub fn extract_pointer_from_vec(mods: &mut Vec<ModuleArc>, i: usize) -> ModulePointer {
    // get unsafe fat pointer
    return extract_pointer(&mut mods[i]);
}

//
// pub(crate) trait DefaultModuleInterface {}
//
// type DefaultModulePointer = Option<NonNull<dyn DefaultModuleInterface>>;
//
//
// pub(crate) trait DefaultModule {
//     fn defult_module_interface() -> DefaultModulePointer;
// }

//Specialized for audio only for executing worker thread in it
// macro_rules! is_default_module {
//     ($($t:ty),+ $(,)?) => ($(
//         impl DefaultModule for $t {
//             fn jobs(&self) -> Box<De> {
//                 &self.defa
//             }
//
//             fn jobs_mut(&mut self) -> &mut Vec<String> {
//                 &mut self.jobs
//             }
//         }
//     )+)
// }
