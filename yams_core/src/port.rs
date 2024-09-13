use std::cell::UnsafeCell;
use std::sync::Arc;
use std::vec::Vec;

const CHANELS: usize = 16;

#[derive(Clone)]
pub struct AudioPort {
    pub value: [f32; CHANELS],
    pub count: usize,
    pub name: String,
}

impl AudioPort {
    pub fn new(name: &str) -> Self {
        AudioPort {
            value: [0.0; CHANELS],
            count: 1,
            name: name.to_string(),
        }
    }

    pub fn create_audio_ports(names: Vec<&str>) -> Vec<AudioPort> {
        let res: Vec<AudioPort> = names.into_iter().map(|name| AudioPort::new(name)).collect();
        res
    }

    pub fn create_n_audio_ports(n: usize, name: &str) -> Vec<AudioPort> {
        let digits: Vec<usize> = (0..n).into_iter().collect();
        let res = digits.into_iter().map(|n| AudioPort::new(
            format!("{name}_{n}").as_str())).collect();
        res
    }
}

pub type AudioPortsCell = Arc<UnsafeCell<Vec<AudioPort>>>;

// just way to share unsafe pointer to vector
#[derive(Copy, Clone)]
pub struct UnsafeAudioPorts(pub *mut Vec<AudioPort>);

unsafe impl Send for UnsafeAudioPorts {}

unsafe impl Sync for UnsafeAudioPorts {}


pub enum ParameterType {
    F64(f64),
    Bool(bool),
    String(String),
    I64(i64),
}

pub struct Parameter {
    pub value: ParameterType,
    pub name: String,
    // todo add moar stuff
}


impl Parameter {
    pub fn new(parameter_type: ParameterType, name: &str) -> Self {
        Parameter {
            value: parameter_type,
            name: name.to_string(),
        }
    }
}
