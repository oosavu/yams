extern crate anyhow;
extern crate cpal;
extern crate ringbuf;

use crate::module::*;
use crate::port::*;
use crate::synth_core::RealTimeCoreArc;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::StreamError;
use ringbuf::HeapRb;
use std::sync::{Arc, Mutex};

pub struct CPALAudioDriver {
    input_stream: Option<cpal::Stream>,
    output_stream: Option<cpal::Stream>,
    #[allow(unused)]
    host: cpal::Host,
    input_device: cpal::Device,
    output_device: cpal::Device,
    input_config: cpal::StreamConfig,
    output_config: cpal::StreamConfig,
    to_engine: UnsafeAudioPorts,
    from_engine: UnsafeAudioPorts,
}

impl CPALAudioDriver {
    pub fn create(to_engine: UnsafeAudioPorts, from_engine: UnsafeAudioPorts) -> AudioDriverArc {
        let host = cpal::default_host();

        let input_device = host.default_input_device().unwrap();
        let input_config = input_device.default_input_config().unwrap().config();
        println!("Default input config: {:?}", &input_config);

        let output_device = host.default_output_device().unwrap();
        let output_config = output_device.default_output_config().unwrap().config();
        assert!(output_config.channels > 0);
        println!("Default output config: {:?}", &output_config);

        #[allow(clippy::arc_with_non_send_sync)]
        return Arc::new(Mutex::new(CPALAudioDriver {
            input_stream: None,
            output_stream: None,
            host,
            input_device,
            output_device,
            input_config,
            output_config,
            to_engine,
            from_engine,
        }));
    }
}

impl CPALAudioDriver {}

impl AudioDriver for CPALAudioDriver {
    fn recommended_framerate(&self) -> cpal::SampleRate {
        self.input_config.sample_rate
    }
    fn start_process(&mut self, rt_core: RealTimeCoreArc) {
        let input_channels = self.input_config.channels as usize;
        let buffer = HeapRb::<f32>::new(2048 * input_channels);
        let (mut producer, mut consumer) = buffer.split();

        let output_channels = self.output_config.channels as usize;

        let to_engine_ref = unsafe { self.to_engine.0.as_mut().unwrap() };
        let from_engine_ref = unsafe { self.from_engine.0.as_ref().unwrap() };

        let output_stream = self
            .output_device
            .build_output_stream(
                &self.output_config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let required_frames = data.len() / output_channels;

                    for frame in 0..required_frames {
                        // for i in 0..input_channels {
                        for p in to_engine_ref.iter_mut().take(input_channels) {
                            match consumer.pop() {
                                None => (),
                                Some(val) => p.value[0] = val,
                            }
                        }
                        rt_core.lock().unwrap().compute_frame(1);
                        for i in 0..output_channels {
                            data[frame * output_channels + i] = from_engine_ref[i].value[0];
                        }
                    }
                },
                move |err: StreamError| {
                    eprintln!("an error occurred on output stream: {}", err);
                },
                None,
            )
            .unwrap();

        let input_stream = self
            .input_device
            .build_input_stream(
                &self.input_config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    //for i in 0..data.len() {
                    for d in data {
                        producer.push(*d).unwrap();
                    }
                },
                move |err: StreamError| {
                    eprintln!("an error occurred on input stream: {}", err);
                },
                None,
            )
            .unwrap();

        output_stream.play().unwrap();
        input_stream.play().unwrap();

        self.input_stream = Some(input_stream);
        self.output_stream = Some(output_stream);
    }

    fn stop(&mut self) {
        self.input_stream = None;
        self.output_stream = None;
    }
}
