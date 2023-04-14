#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(split_array)]

use std::f32::EPSILON;
use std::f32::consts::TAU;
use std::sync::Arc;
use std::sync::atomic::{Ordering, AtomicU8};

use real_time_fir_iir_filters::iir::{WahFilter, IIRFilter};
use vst::{prelude::*, plugin_main};

pub mod parameters;

use parameters::*;

const CHANNEL_COUNT: usize = 2;

const CHANGE: f32 = 0.2;
const WAH_DEFAULT: f32 = 0.5;

struct WahPlugin
{
    pub param: Arc<WahParameters>,
    filter: [WahFilter; CHANNEL_COUNT],
    rate: f32
}

impl WahPlugin
{

}

impl Plugin for WahPlugin
{
    fn new(_host: HostCallback) -> Self
    where
        Self: Sized
    {
        WahPlugin {
            param: Arc::new(WahParameters {
                wah: AtomicFloat::from(WAH_DEFAULT)
            }),
            filter: array_init::array_init(|_| WahFilter::new(WAH_DEFAULT)),
            rate: 44100.0
        }
    }

    fn get_info(&self) -> Info
    {
        Info {
            name: "WahVoice".to_string(),
            vendor: "Soma FX".to_string(),
            presets: 0,
            parameters: 1,
            inputs: CHANNEL_COUNT as i32,
            outputs: CHANNEL_COUNT as i32,
            midi_inputs: 0,
            midi_outputs: 0,
            unique_id: 75474563,
            version: 1,
            category: Category::Effect,
            initial_delay: 0,
            preset_chunks: false,
            f64_precision: true,
            silent_when_stopped: true,
            ..Default::default()
        }
    }

    fn set_sample_rate(&mut self, rate: f32)
    {
        self.rate = rate;
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>)
    {
        for (c, (input_channel, output_channel)) in buffer.zip().enumerate()
        {
            self.filter[c].k = CHANGE*self.param.wah.get() + (1.0 - CHANGE)*self.filter[c].k;
            for (input_sample, output_sample) in input_channel.into_iter()
                .zip(output_channel.into_iter())
            {
                let x = *input_sample;
                let y = self.filter[c].filter(self.rate, x)[0];
                *output_sample = y;
            }
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters>
    {
        self.param.clone()
    }
}

plugin_main!(WahPlugin);