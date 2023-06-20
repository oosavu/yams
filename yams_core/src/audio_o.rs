extern crate anyhow;
extern crate cpal;
extern crate ringbuf;

use crate::synth_core::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::StreamError;
use crate::module::*;
use crate::port::*;

use ringbuf::{HeapConsumer, HeapProducer, HeapRb};

struct CpalStuff {
    input_stream: cpal::Stream,
    output_stream: cpal::Stream,
    host: cpal::Host,
    input_device: cpal::Device,
    output_device: cpal::Device,
    input_config: cpal::StreamConfig,
    output_config: cpal::StreamConfig,
}

type Callback = Arc<Mutex<Box<dyn  Fn() + Send + Sync>>>;

pub struct ModuleO {
    ins: Vec<AudioPort>,
    outs: Vec<AudioPort>,
    framerate: i64,
    process_fn: Option<Callback>,
    cpal_instance: Option<Box<CpalStuff>>,
}

#[derive(Copy, Clone)]
struct VecPointerWrapper(*const Vec<AudioPort>);

unsafe impl Send for VecPointerWrapper {}

impl Module for ModuleO {
    fn set_framerate(&mut self, framerate: i64) {
        self.framerate = framerate;
    }
    fn process(&mut self) {
        unsafe {
//             static mut count_i: isize = 0;
//             count_i = count_i + 1;
// //            dbg!(count_i);
//             self.producer.push(self.ins[0].value[0]);
        }
    }
    fn inputs(&mut self) -> &mut Vec<AudioPort> {
        &mut self.ins
    }

    fn outputs(&mut self) -> &mut Vec<AudioPort> {
        &mut self.outs
    }

    fn recommended_framerate(&mut self) -> Option<cpal::SampleRate> {
        return Some(self.cpal_instance.as_ref().unwrap().input_config.sample_rate);
    }

    fn can_be_default_module(&self) -> bool {
        return true;
    }

    fn set_process_fn(&mut self, process_fn: Box<dyn Fn()>) {
        //self.process_fn = process_fn;
    }
}

impl ModuleO {
    fn error_fn(err: StreamError) {
        eprintln!("an error occurred on stream: {}", err);
    }

    fn reinit(&mut self) {
        let host = cpal::default_host();
        let buffer = HeapRb::new(512);
        let (mut producer, mut consumer) = buffer.split();

        let output_device = host.default_output_device().unwrap();
        let output_config = output_device.default_output_config().unwrap().into();
        println!("Default output config: {:?}", &output_config);
        let output_stream = output_device
            .build_output_stream(
                &output_config,
                move |data: &mut [f32], info: &cpal::OutputCallbackInfo| {
                    Self::output_fn(&mut consumer, data, info)
                },
                &Self::error_fn,
                None,
            ).unwrap();

        let input_device = host.default_input_device().unwrap();
        let input_config = input_device.default_input_config().unwrap().into();
        println!("Default input config: {:?}", &input_config);

        let engine_fn = self.process_fn;
        let data_in = VecPointerWrapper(&self.ins as *const Vec<AudioPort>);
        let data_out = VecPointerWrapper(&self.outs as *const Vec<AudioPort>);
        //let arc_pointer_out = Arc::new(Mutex::new(&mut self.outs));
        let input_stream = input_device
            .build_input_stream(
                &input_config,
                move |data: &[f32], info: &cpal::InputCallbackInfo| {
                    Self::input_fn(&mut producer,
                                   data,
                                   info,
                                   input_config.channels,
                                   output_config.channels,
                                   data_in,
                                   data_out,
                                   engine_fn)
                },
                &Self::error_fn,
                None,
            ).unwrap();

        output_stream.play().unwrap();
        input_stream.play().unwrap();


        self.cpal_instance = Some(Box::new(CpalStuff {
            input_stream,
            output_stream,
            host,
            input_device,
            output_device,
            input_config,
            output_config,
        }));
    }

    fn output_fn(
        consumer: &mut HeapConsumer<f32>,
        data: &mut [f32],
        calback_info: &cpal::OutputCallbackInfo,
    ) {
        unsafe {
            static mut COUNT: isize = 0;
            static mut COUNT_SAMPLES: usize = 0;
            static mut SILENTS_FRAMES: usize = 0;
            COUNT = COUNT + 1;
            //TODO can we memcpy?
            let mut input_fell_behind = false;
            //COUNT_SAMPLES = COUNT_SAMPLES + data.len();
            for sample in data {
                *sample = match consumer.pop() {
                    Some(s) => {
                        //dbg!(s);
                        COUNT_SAMPLES = COUNT_SAMPLES + 1;
                        s
                    }
                    None => {
                        //println!("beha");
                        input_fell_behind = true;
                        SILENTS_FRAMES = SILENTS_FRAMES + 1;
                        0.0
                    }
                };
            }
            if input_fell_behind {
                eprintln!("input stream fell behind: try increasing latency");
            }

            dbg!(COUNT, COUNT_SAMPLES, SILENTS_FRAMES);
        }
    }

    fn input_fn(
        producer: &mut HeapProducer<f32>,
        data: &[f32],
        calback_info: &cpal::InputCallbackInfo,
        input_channels: cpal::ChannelCount,
        output_channels: cpal::ChannelCount,
        ins: VecPointerWrapper,
        outs: VecPointerWrapper,
        process_fn:Callback,
    ) {
        unsafe {
            // let mut input_fell_behind = false;
            // //COUNT_SAMPLES = COUNT_SAMPLES + data.len();
            // let in_samples_count = data.len() / input_channels;
            // for i in 0..in_samples_count {
            //     for j in 0..input_channels {
            //         *ins[j] = data[i * input_channels + j];
            //     }
            //     process_fn();
            //     for j in 0..output_channels {
            //         producer.push(*outs[j])?;
            //     }
            // }
        }
    }
}

impl Default for ModuleO {
    fn default() -> Self {
        ModuleO {
            ins: AudioPort::create_audio_ports(8),
            outs: AudioPort::create_audio_ports(8),
            framerate: 0,
            process_fn: None,
            cpal_instance: None,
        }
    }
}
