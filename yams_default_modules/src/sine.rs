use yams_core::*;

pub struct ModuleSine {
    ins: Vec<AudioPort>,
    outs: Vec<AudioPort>,
    sample_clock: f32,
    framerate: f64,
}

impl Module for ModuleSine {
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

    fn audio_driver(&self) -> Option<AudioDriverArc> {
        None
    }
}

pub struct ModuleSineFabric {
    info: ModuleInfo,
}

impl ModuleFabric for ModuleSineFabric {
    fn create(&self) -> ModuleArc {
        Arc::new(Mutex::new(ModuleSine::default()))
    }
    fn info(&self) -> &ModuleInfo {
        &self.info
    }
}

impl Default for ModuleSineFabric {
    fn default() -> Self {
        ModuleSineFabric {
            info: ModuleInfo {
                name: "sine".to_string(),
                inputs: vec![PortInfo { name: "freq".to_string(), channels: 1 },
                ],
                outputs: vec![PortInfo { name: "out".to_string(), channels: 1 },
                ],
            },
        }
    }
}

impl Default for ModuleSine {
    fn default() -> Self {
        ModuleSine {
            ins: AudioPort::create_audio_ports(1),
            outs: AudioPort::create_audio_ports(1),
            sample_clock: 0.0,
            framerate: 0.0f64,
        }
    }
}
