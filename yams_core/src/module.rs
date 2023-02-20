use std::sync::{Arc, Mutex};
use std::ptr::NonNull;
use std::borrow::BorrowMut;
use crate::AudioPort;
use crate::Cable;

pub type ModuleArc = Arc<Mutex<dyn Module>>;
pub type ModulePointer = Option<NonNull<dyn Module>>;

pub trait Module {
    //fn hand_inputs(&mut self) -> &mut Vec<Port>;
    //fn hand_outputs(&mut self) -> &mut Vec<Port>;
    fn set_framerate(&mut self, framerate: i64);
    fn process(&mut self);
    fn inputs(&mut self) -> &mut Vec<AudioPort>;
    fn outputs(&mut self) -> &mut Vec<AudioPort>;

    fn recommended_framerate(&mut self) -> Option<i64>;
    fn can_be_default_module() -> bool;
    fn set_pocess_fn(&mut self, process_fn: Option(fn(int)));
}

// extract unsafe fat pointer
pub fn extract_pointer(module: &mut ModuleArc) -> ModulePointer {
    return unsafe {
        let asd: &Mutex<dyn Module> = module.borrow_mut();
        let qwe: *mut dyn Module = &mut *asd.lock().unwrap() as *mut dyn Module;
        Some(NonNull::new_unchecked(qwe))
    };
}

pub fn extract_pointer_from_vec(mods: &mut Vec<ModuleArc>, i: usize) -> ModulePointer { // get unsafe fat pointer
    return extract_pointer(&mut mods[i])
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