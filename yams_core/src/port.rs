use crate::ParameterType;
use crate::{ParameterInfo, ParameterInfoVec, PortInfoVec};
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
    pub fn create(infos: &PortInfoVec) -> Vec<Self> {
        let res: Vec<Self> = infos
            .into_iter()
            .map(|info| Self {
                value: [0.0; CHANELS],
                count: info.channels as usize,
                name: info.name.to_string(),
            })
            .collect();
        res
    }
}

pub type AudioPortsCell = Arc<UnsafeCell<Vec<AudioPort>>>;

// just way to share unsafe pointer to vector
#[derive(Copy, Clone)]
pub struct UnsafeAudioPorts(pub *mut Vec<AudioPort>);

unsafe impl Send for UnsafeAudioPorts {}

unsafe impl Sync for UnsafeAudioPorts {}

pub struct Parameter {
    pub value: ParameterType,
    pub info: ParameterInfo,
}

impl Parameter {
    pub fn new(parameter_type: ParameterType, info: &ParameterInfo) -> Self {
        Parameter {
            value: parameter_type,
            info: info.clone(),
        }
    }

    pub fn create(infos: &ParameterInfoVec) -> Vec<Self> {
        let res: Vec<Self> = infos
            .into_iter()
            .map(|info| Parameter {
                value: info.parameter_type.clone(),
                info: info.clone(),
            })
            .collect();
        res
    }
}
