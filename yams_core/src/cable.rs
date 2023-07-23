use crate::module::*;

pub struct Cable {
    pub input_module_p: ModulePointer,
    pub output_module_p: ModulePointer,
    pub input_port: usize,
    pub output_port: usize,
}

impl Cable {
    pub fn new_cable(input_m: &mut ModuleArc, output_m: &mut ModuleArc, input_port: usize, output_port: usize) -> Self
    {
        Cable {
            input_module_p: extract_pointer(input_m),
            output_module_p: extract_pointer(output_m),
            input_port: input_port,
            output_port: output_port,
        }
    }
}