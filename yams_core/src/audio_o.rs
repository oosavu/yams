use crate::cpal_audio_driver::*;
use crate::module::*;
use crate::port::*;
use std::cell::UnsafeCell;
use std::ops::Deref;
use std::sync::Arc;

pub struct ModuleO {
    ins: AudioPortsCell,
    outs: AudioPortsCell,
    framerate: f64,
    cpal_instance: Option<AudioDriverArc>,
}

impl Module for ModuleO {
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
}

impl Default for ModuleO {
    fn default() -> Self {
        #[allow(clippy::arc_with_non_send_sync)]
        let ins_ports = Arc::new(UnsafeCell::new(AudioPort::create_audio_ports(8)));
        #[allow(clippy::arc_with_non_send_sync)]
        let outs_ports = Arc::new(UnsafeCell::new(AudioPort::create_audio_ports(8)));
        //let ins_ports = UnsafeCell::new(AudioPort::create_audio_ports(8));
        //let outs_ports = UnsafeCell::new(AudioPort::create_audio_ports(8));

        let mut res = ModuleO {
            ins: ins_ports,
            outs: outs_ports,
            framerate: 0.0f64,
            cpal_instance: None,
        };
        res.cpal_instance = Some(CPALAudioDriver::create(
            UnsafeAudioPorts(res.outs.get()),
            UnsafeAudioPorts(res.ins.get()),
        ));
        res
    }
}
