use crate::module::*;
use crate::port::*;
use crate::cpal_audio_driver::*;

pub struct ModuleO {
    ins: Vec<AudioPort>,
    outs: Vec<AudioPort>,
    framerate: i64,
    cpal_instance: Option<AudioDriverArc>,
}

impl Module for ModuleO {
    fn set_framerate(&mut self, framerate: i64) {
        self.framerate = framerate;
    }
    fn process(&mut self) {}
    fn inputs(&mut self) -> &mut Vec<AudioPort> {
        &mut self.ins
    }

    fn outputs(&mut self) -> &mut Vec<AudioPort> {
        &mut self.outs
    }

    fn audio_driver(&self) -> Option<AudioDriverArc> {
        return self.cpal_instance.clone();
    }
}

impl Default for ModuleO {
    fn default() -> Self {
        let ins_ports = AudioPort::create_audio_ports(8);
        let outs_ports = AudioPort::create_audio_ports(8);

        ModuleO {
            ins: ins_ports,
            outs: outs_ports,
            framerate: 0,
            cpal_instance: Some(CPALAudioDriver::create(UnsafeAudioPorts(&ins_ports), UnsafeAudioPorts(&outs_ports))),
        }
    }
}
