use crate::{AudioPort, ModuleArc};
use cpal::ChannelCount;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct PortInfo {
    pub name: String,
    pub channels: i32,
}

impl PortInfo {
    pub fn create_vec(name: &str, n: usize, channel_count: i32) -> Vec<Self> {
        let digits: Vec<usize> = (0..n).into_iter().collect();
        let res = digits
            .into_iter()
            .map(|n| PortInfo {
                name: format!("{name}_{n}"),
                channels: channel_count,
            })
            .collect();
        res
    }
}

pub type PortInfoVec = Vec<PortInfo>;

#[derive(Clone)]
pub enum ParameterType {
    F64(f64),
    Bool(bool),
    String(String),
    I64(i64),
}

#[derive(Clone)]
pub struct ParameterInfo {
    pub name: String,
    pub parameter_type: ParameterType,
}

pub type ParameterInfoVec = Vec<ParameterInfo>;

#[derive(Clone)]
pub struct ModuleInfo {
    pub name: String,
    pub inputs: PortInfoVec,
    pub outputs: PortInfoVec,
    pub parameters: ParameterInfoVec,
}

pub trait ModuleFabric {
    fn info(&self) -> &ModuleInfo;
    fn create(&self) -> ModuleArc;
}

pub trait ModulesRegistry {
    fn fabrics(&self) -> &HashMap<String, Box<dyn ModuleFabric>>;
}
