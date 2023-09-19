pub use std::sync::atomic::{AtomicBool, Ordering};
pub use std::sync::{Arc, Condvar, Mutex};
pub use std::{thread, time};
pub use std::ptr::NonNull;
use std::time::{Duration, SystemTime};
use crate::module::*;
use crate::cable::*;

const FALLBACK_FRAME_SIZE: usize = 64;

// low level structure for working with raw pointers
struct RealTimeCore {
    pub modules_pointers: Vec<ModulePointer>,
    pub default_module: ModulePointer,
    pub cable_core: Vec<Cable>,
    pub sample_rate: i64,
    pub current_time: SystemTime,
    alive: Arc<AtomicBool>,
    is_fallback_active: Arc<(Mutex<bool>, Condvar)>,
}

unsafe impl Send for RealTimeCore {}
unsafe impl Sync for RealTimeCore {}

impl RealTimeCore {
    pub fn compute_frame(&mut self, time_frame: usize) { // TODO does it vectorized automativally?
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

pub struct Engine {
    modules: Vec<ModuleArc>,
    //cables: Vec<Mutex<Cable>>,
    core: Arc<Mutex<RealTimeCore>>,

    fallback_mutex: Arc<(Mutex<bool>, Condvar)>,
    frame_rate: i64,

    fallback_handle: Option<thread::JoinHandle<()>>,
    fallback_alive: Arc<AtomicBool>, // alive of tread itself
}

impl Engine {
    pub fn start(&mut self) {
        let def_module = self.default_module();
        match def_module {
            None => {
                self.start_fallback();
            }
            Some(m) =>{
                //m.lock().unwrap().set_process_fn()

            }
        }

    }

    pub fn stop(&mut self) {
        self.stop_fallback();
    }

    fn default_module(&mut self) -> Option<&ModuleArc> {
        return self.modules.iter().find(|m|{m.lock().unwrap().audio_driver().is_some()})
    }

    pub fn add_module(&mut self, module: &mut ModuleArc){
        self.modules.push(module.clone());
        let mut cor = self.core.lock().unwrap();
        cor.modules_pointers.push(extract_pointer(&module));
    }

    pub fn add_cable(&mut self,  cable: Cable){
        //self.cables.push(cable);
        let mut cor = self.core.lock().unwrap();
        cor.cable_core.push(cable);
    }

    fn start_fallback(&mut self) {
        dbg!("starting fallback...");
        self.fallback_alive.store(true, Ordering::SeqCst);
        let mut alive = self.fallback_alive.clone();
        let cor = self.core.clone();
        self.fallback_handle = Some(thread::spawn(move || {

            alive.store(true, Ordering::SeqCst);
            {
                let mut cor = cor.lock().unwrap();
                cor.sample_rate = 48000;
                cor.current_time = SystemTime::now();
            }
            let mut samples_count: i64 = 0;
            while alive.load(Ordering::SeqCst) {
                let mut cor = cor.lock().unwrap();
                let duration = cor.current_time.elapsed().unwrap().as_millis() as i64;
                let required_samples = cor.sample_rate * duration / 1000;
                if required_samples < samples_count
                {
                    let pause_millis = (samples_count - required_samples) * 1000i64 / cor.sample_rate;
                    //dbg!(pause_millis);
                    thread::sleep(Duration::from_millis(std::cmp::max(pause_millis as u64, 1)));
                    continue;
                }
                cor.compute_frame(FALLBACK_FRAME_SIZE);
                samples_count = samples_count + FALLBACK_FRAME_SIZE as i64;
              //  dbg!(samples_count);
                //thread::sleep(time::Duration::from_millis(10));
            }
            dbg!("end circle");
        }));
    }

    fn stop_fallback(&mut self) {
        self.fallback_alive.store(false, Ordering::SeqCst);
        dbg!("qweqwe");
        self.fallback_handle
            .take().expect("Called stop on non-running thread")
            .join().expect("Could not join spawned thread");
        let cor = self.core.lock().unwrap();
        dbg!("fallback stopped. working time: {}", cor.current_time.elapsed());
    }
}

impl Default for Engine{
    fn default() -> Self {
        Engine{
            modules: vec![],
            core: Arc::new(Mutex::new(RealTimeCore {
                modules_pointers: vec![],
                default_module: None,
                cable_core: vec![],
                sample_rate: 0,
                current_time: SystemTime::now(),
                alive: Arc::new(Default::default()),
                is_fallback_active: Arc::new((Mutex::new(false), Default::default()))
            })),
            fallback_mutex: Arc::new((Mutex::new(false), Default::default())),
            frame_rate: 0,
            fallback_handle: None,
            fallback_alive: Arc::new(Default::default())
        }
    }
}
//
// pub fn test_engine() -> Engine {
//     let mut mods: Vec<Arc<Mutex<dyn Module>>> = vec![Arc::new(Mutex::new(sine::ModuleSine::default())),
//                                                      Arc::new(Mutex::new(audio_o::ModuleO::default()))];
//     let alive: Arc<AtomicBool> = Arc::new(AtomicBool::default());
//     let fallback_active: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(true), Condvar::new()));
//     Engine {
//         fallback_handle: None,
//         fallback_alive: alive.clone(),
//         core: Arc::new(Mutex::new(RealTimeCore {
//             modules_pointers: vec![crate::module::extract_pointer_from_vec(&mut mods, 0), crate::module::extract_pointer_from_vec(&mut mods, 1)],
//             default_module: None,
//             cable_core: vec![Cable {
//                 input_module_p: crate::module::extract_pointer_from_vec(&mut mods, 0),
//                 output_module_p: crate::module::extract_pointer_from_vec(&mut mods, 1),
//                 input_port: 0,
//                 output_port: 0,
//             }],
//             sample_rate: 96000,
//             current_time: SystemTime::now(),
//             alive: alive.clone(),
//             is_fallback_active: fallback_active.clone(),
//         })),
//         fallback_mutex: fallback_active.clone(),
//         cables: vec![Mutex::new(Cable {
//             input_module_p: crate::module::extract_pointer_from_vec(&mut mods, 0),
//             output_module_p: crate::module::extract_pointer_from_vec(&mut mods, 1),
//             input_port: 0,
//             output_port: 0,
//         })],
//         modules: mods,
//         frame_rate: 48000,
//     }
// }