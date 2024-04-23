pub struct PortInfo {
    pub name: String,
    pub channels: i32,
}

pub struct ModuleInfo {
    pub name: String,
    pub inputs: Vec<PortInfo>,
    pub outputs: Vec<PortInfo>,
}
