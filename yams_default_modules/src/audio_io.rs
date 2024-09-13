use crate::cpal_audio_driver::*;
use std::cell::UnsafeCell;
use std::ops::Deref;
use std::sync::Arc;
use yams_core::*;

pub struct ModuleIO {
    info: ModuleInfo,
    ins: AudioPortsCell,
    outs: AudioPortsCell,
    parameters: Vec<Parameter>,
    framerate: f64,
    cpal_instance: Option<AudioDriverArc>,
}

impl Module for ModuleIO {
    fn set_framerate(&mut self, framerate: f64) {
        self.framerate = framerate;
    }
    fn process(&mut self) {}
    fn inputs(&mut self) -> &mut Vec<AudioPort> {
        return unsafe { self.ins.deref().get().as_mut().unwrap() };
    }

    fn outputs(&mut self) -> &mut Vec<AudioPort> {
        return unsafe { self.outs.deref().get().as_mut().unwrap() };
    }

    fn audio_driver(&self) -> Option<AudioDriverArc> {
        self.cpal_instance.clone()
    }

    fn parameters(&mut self) -> &mut Vec<Parameter> {
        return self.parameters.as_mut();
    }

    fn info(&self) -> &ModuleInfo {
        &self.info
    }
}

pub struct ModuleIOFabric {
    info: ModuleInfo,
}

impl Default for ModuleIOFabric {
    fn default() -> Self {
        Self {
            info: ModuleInfo {
                name: "audio_io".to_string(),
                inputs: PortInfo::create_vec("in", 8, 1),
                outputs: PortInfo::create_vec("out", 8, 1),
                parameters: vec![],
            },
        }
    }
}

impl ModuleFabric for ModuleIOFabric {
    fn info(&self) -> &ModuleInfo {
        &self.info
    }

    fn create(&self) -> ModuleArc {
        #[allow(clippy::arc_with_non_send_sync)]
        let ins_ports = Arc::new(UnsafeCell::new(AudioPort::create(&self.info.inputs)));
        #[allow(clippy::arc_with_non_send_sync)]
        let outs_ports = Arc::new(UnsafeCell::new(AudioPort::create(&self.info.outputs)));

        let mut res = ModuleIO {
            info: self.info.clone(),
            ins: ins_ports,
            outs: outs_ports,
            parameters: Parameter::create(&self.info.parameters),
            framerate: 0.0f64,
            cpal_instance: None,
        };

        res.cpal_instance = Some(CPALAudioDriver::create(
            UnsafeAudioPorts(res.outs.get()),
            UnsafeAudioPorts(res.ins.get()),
        ));

        Arc::new(Mutex::new(ModuleIO {
            info: self.info.clone(),
            ins: Arc::new(UnsafeCell::new(AudioPort::create(&self.info.inputs))),
            outs: Arc::new(UnsafeCell::new(AudioPort::create(&self.info.outputs))),
            parameters: Parameter::create(&self.info.parameters),
            framerate: 0.0,
            cpal_instance: None,
        }))
    }
}
