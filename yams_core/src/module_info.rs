

pub struct PortInfo{
    name: String,
    channels: i32,
}

pub struct ModuleInfo{
    name: String,
    inputs: Vec<PortInfo>,
    outputs: Vec<PortInfo>,
}