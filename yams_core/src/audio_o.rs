use crate::cpal_audio_driver::*;
use crate::module::*;
use crate::port::*;
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

pub struct ModuleO {
    ins: AudioPortsCell,
    outs: AudioPortsCell,
    framerate: i64,
    cpal_instance: Option<AudioDriverArc>,
}

impl Module for ModuleO {
    fn set_framerate(&mut self, framerate: i64) {
        self.framerate = framerate;
    }
    fn process(&mut self) {}
    fn inputs(&mut self) -> &mut Vec<AudioPort> {
        let mut qwe = self.ins.deref();
        let mut asd = qwe.get();
        return unsafe { asd.as_mut().unwrap() };
    }

    fn outputs(&mut self) -> &mut Vec<AudioPort> {
        let mut qwe = self.outs.deref();
        let mut asd = qwe.get();
        return unsafe { asd.as_mut().unwrap() };
    }

    fn audio_driver(&self) -> Option<AudioDriverArc> {
        return self.cpal_instance.clone();
    }
}

impl Default for ModuleO {
    fn default() -> Self {
        let ins_ports = Arc::new(UnsafeCell::new(AudioPort::create_audio_ports(8)));
        let outs_ports = Arc::new(UnsafeCell::new(AudioPort::create_audio_ports(8)));
        //let ins_ports = UnsafeCell::new(AudioPort::create_audio_ports(8));
        //let outs_ports = UnsafeCell::new(AudioPort::create_audio_ports(8));

        let mut res = ModuleO {
            ins: ins_ports,
            outs: outs_ports,
            framerate: 0,
            cpal_instance: None,
        };
        res.cpal_instance = Some(CPALAudioDriver::create(
            UnsafeAudioPorts(res.outs.get()),
            UnsafeAudioPorts(res.ins.get()),
        ));
        return res;
    }
}
