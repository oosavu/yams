use crate::cpal_audio_driver::*;
use std::cell::UnsafeCell;
use std::ops::Deref;
use std::sync::Arc;
use yams_core::*;

pub struct ModuleIO {
    ins: AudioPortsCell,
    outs: AudioPortsCell,
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
}

pub struct ModuleIOFabric {
    info: ModuleInfo,
}

impl ModuleFabric for ModuleIOFabric {
    fn info(&self) -> &ModuleInfo {
        &self.info
    }

    fn create(&self) -> ModuleArc {
        Arc::new(Mutex::new(ModuleIO::default()))
    }
}

impl Default for ModuleIOFabric {
    fn default() -> Self {
        ModuleIOFabric {
            info: ModuleInfo {
                name: "audio_io".to_string(),
                inputs: vec![PortInfo { name: "in0".to_string(), channels: 1 },
                             PortInfo { name: "in1".to_string(), channels: 1 },
                             PortInfo { name: "in2".to_string(), channels: 1 },
                             PortInfo { name: "in3".to_string(), channels: 1 },
                             PortInfo { name: "in4".to_string(), channels: 1 },
                             PortInfo { name: "in5".to_string(), channels: 1 },
                             PortInfo { name: "in6".to_string(), channels: 1 },
                             PortInfo { name: "in7".to_string(), channels: 1 },
                ],
                outputs: vec![PortInfo { name: "out0".to_string(), channels: 1 },
                              PortInfo { name: "out1".to_string(), channels: 1 },
                              PortInfo { name: "out2".to_string(), channels: 1 },
                              PortInfo { name: "out3".to_string(), channels: 1 },
                              PortInfo { name: "out4".to_string(), channels: 1 },
                              PortInfo { name: "out5".to_string(), channels: 1 },
                              PortInfo { name: "out6".to_string(), channels: 1 },
                              PortInfo { name: "out7".to_string(), channels: 1 },
                ],
            },
        }
    }
}

impl Default for ModuleIO {
    fn default() -> Self {
        #[allow(clippy::arc_with_non_send_sync)]
            let ins_ports = Arc::new(UnsafeCell::new(AudioPort::create_audio_ports(8)));
        #[allow(clippy::arc_with_non_send_sync)]
            let outs_ports = Arc::new(UnsafeCell::new(AudioPort::create_audio_ports(8)));
        //let ins_ports = UnsafeCell::new(AudioPort::create_audio_ports(8));
        //let outs_ports = UnsafeCell::new(AudioPort::create_audio_ports(8));

        let mut res = ModuleIO {
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
