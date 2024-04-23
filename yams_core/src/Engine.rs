pub use std::sync::atomic::{AtomicBool, Ordering};
pub use std::sync::{Arc, Condvar, Mutex};
pub use std::thread;
use std::time::{Duration, SystemTime};
use crate::{Cable, extract_pointer, ModuleArc, RealTimeCore, RealTimeCoreArc};

const FALLBACK_FRAME_SIZE: usize = 64;

pub struct Engine {
    modules: Vec<ModuleArc>,
    //cables: Vec<Mutex<Cable>>,
    core: RealTimeCoreArc,

    #[allow(unused)]
    fallback_mutex: Arc<(Mutex<bool>, Condvar)>,
    frame_rate: f64,

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
            Some(m) => {
                let c = self.core.clone();
                let driver_arc = m.lock().unwrap().audio_driver().unwrap();
                let mut driver = driver_arc.lock().unwrap();
                for modul in self.modules.iter_mut() {
                    modul
                        .lock()
                        .unwrap()
                        .set_framerate(driver.recommended_framerate().0 as f64);
                }
                self.frame_rate = driver.recommended_framerate().0 as f64;
                driver.start_process(c);
            }
        }
    }

    pub fn stop(&mut self) {
        let def_module = self.default_module();
        match def_module {
            None => {
                self.stop_fallback();
            }
            Some(m) => {
                m.lock()
                    .unwrap()
                    .audio_driver()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .stop();
            }
        }
    }

    fn default_module(&self) -> Option<&ModuleArc> {
        return self
            .modules
            .iter()
            .find(|m| m.lock().unwrap().audio_driver().is_some());
    }

    pub fn add_module(&mut self, module: &mut ModuleArc) {
        self.modules.push(module.clone());
        let mut cor = self.core.lock().unwrap();
        cor.modules_pointers.push(extract_pointer(module));
    }

    pub fn add_cable(&mut self, cable: Cable) {
        //self.cables.push(cable);
        let mut cor = self.core.lock().unwrap();
        cor.cable_core.push(cable);
    }

    fn start_fallback(&mut self) {
        dbg!("starting fallback...");
        self.fallback_alive.store(true, Ordering::SeqCst);
        let alive = self.fallback_alive.clone();
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
                if required_samples < samples_count {
                    let pause_millis =
                        (samples_count - required_samples) * 1000i64 / cor.sample_rate;
                    //dbg!(pause_millis);
                    thread::sleep(Duration::from_millis(std::cmp::max(pause_millis as u64, 1)));
                    continue;
                }
                cor.compute_frame(FALLBACK_FRAME_SIZE);
                samples_count += FALLBACK_FRAME_SIZE as i64;
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
            .take()
            .expect("Called stop on non-running thread")
            .join()
            .expect("Could not join spawned thread");
        let cor = self.core.lock().unwrap();
        dbg!(
            "fallback stopped. working time: {}",
            cor.current_time.elapsed().unwrap()
        );
    }
}

impl Default for Engine {
    fn default() -> Self {
        Engine {
            modules: vec![],
            core: Arc::new(Mutex::new(RealTimeCore {
                modules_pointers: vec![],
                default_module: None,
                cable_core: vec![],
                sample_rate: 0,
                current_time: SystemTime::now(),
                alive: Arc::new(Default::default()),
                is_fallback_active: Arc::new((Mutex::new(false), Default::default())),
            })),
            fallback_mutex: Arc::new((Mutex::new(false), Default::default())),
            frame_rate: 0.0f64,
            fallback_handle: None,
            fallback_alive: Arc::new(Default::default()),
        }
    }
}


//
// pub fn test_engine() -> Engine {
//     let mut mods: Vec<Arc<Mutex<dyn Module>>> = vec![Arc::new(Mutex::new(sine::ModuleSine::default())),
//                                                      Arc::new(Mutex::new(audio_o::ModuleIO::default()))];
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
