use std::vec::Vec;

const CHANELS: usize = 16;

#[derive(Clone, Copy)]
pub struct AudioPort {
    pub value: [f32;CHANELS],
    pub count: usize, // TODO make it const
}
impl Default for AudioPort{
    fn default() -> Self {
        return AudioPort{
            value: [0.0; CHANELS],
            count: 1
        }
    }
}
impl AudioPort{
    pub fn create_audio_ports(n: usize) -> Vec<AudioPort>{
        return vec![AudioPort::default(); n];
    }
}

// just way to share unsafe pointer to vector
#[derive(Copy, Clone)]
pub struct UnsafeAudioPorts(pub *const Vec<AudioPort>);
unsafe impl Send for UnsafeAudioPorts {}
unsafe impl Sync for UnsafeAudioPorts {}

pub struct HandPort {
    pub value: f32,
}