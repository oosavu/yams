use crate::audio_io::ModuleIO;
use crate::cpal_audio_driver::CPALAudioDriver;
use std::cell::UnsafeCell;
use yams_core::*;

pub struct ModuleSine {
    ins: Vec<AudioPort>,
    outs: Vec<AudioPort>,
    parameters: Vec<Parameter>,
    sample_clock: f32,
    framerate: f64,
    info: ModuleInfo,
}

impl Module for ModuleSine {
    fn info(&self) -> &ModuleInfo {
        &self.info
    }
    fn set_framerate(&mut self, framerate: f64) {
        self.framerate = framerate;
    }

    fn process(&mut self) {
        self.sample_clock += 1.0;

        let freq = 261.6256 * 2.0f32.powf(self.ins[0].value[0]);
        self.outs[0].value[0] =
            (self.sample_clock * freq * 2.0 * std::f32::consts::PI / self.framerate as f32).sin();
    }
    fn inputs(&mut self) -> &mut Vec<AudioPort> {
        &mut self.ins
    }
    fn outputs(&mut self) -> &mut Vec<AudioPort> {
        &mut self.outs
    }

    fn parameters(&mut self) -> &mut Vec<Parameter> {
        &mut self.parameters
    }

    fn audio_driver(&self) -> Option<AudioDriverArc> {
        None
    }
}

pub struct ModuleSineFabric {
    info: ModuleInfo,
}

impl Default for ModuleSineFabric {
    fn default() -> Self {
        Self {
            info: ModuleInfo {
                name: "sine".to_string(),
                inputs: vec![PortInfo {
                    name: "freq".to_string(),
                    channels: 1,
                }],
                outputs: vec![PortInfo {
                    name: "out".to_string(),
                    channels: 1,
                }],
                parameters: vec![ParameterInfo {
                    name: "freq".to_string(),
                    parameter_type: ParameterType::F64(0.0),
                }],
            },
        }
    }
}

impl ModuleFabric for ModuleSineFabric {
    fn info(&self) -> &ModuleInfo {
        &self.info
    }

    fn create(&self) -> ModuleArc {
        Arc::new(Mutex::new(ModuleSine {
            info: self.info.clone(),
            ins: AudioPort::create(&self.info.inputs),
            outs: AudioPort::create(&self.info.outputs),
            parameters: Parameter::create(&self.info.parameters),
            framerate: 0.0,
            sample_clock: 0.0,
        }))
    }
}
