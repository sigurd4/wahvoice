use std::sync::atomic::{AtomicU8, Ordering};

use vst::prelude::PluginParameters;
use vst::util::AtomicFloat;

const WAH_MIN: f32 = 0.01;
const WAH_MAX: f32 = 1.0;

pub struct WahParameters
{
    pub wah: AtomicFloat
}

impl PluginParameters for WahParameters
{
    fn get_parameter_label(&self, index: i32) -> String
    {
        match index
        {
            0 => "%".to_string(),
            _ => "".to_string()
        }
    }

    fn get_parameter_text(&self, index: i32) -> String
    {
        match index
        {
            0 => format!("{:.3}", 100.0*self.get_parameter(index)),
            _ => "".to_string()
        }
    }

    fn get_parameter_name(&self, index: i32) -> String
    {
        match index
        {
            0 => "Wah".to_string(),
            _ => "".to_string()
        }
    }

    /// Get the value of parameter at `index`. Should be value between 0.0 and 1.0.
    fn get_parameter(&self, index: i32) -> f32
    {
        match index
        {
            0 => (self.wah.get().ln() - WAH_MIN.ln())/(WAH_MAX.ln() - WAH_MIN.ln()),
            _ => 0.0
        }
    }
    
    fn set_parameter(&self, index: i32, value: f32)
    {
        match index
        {
            0 => self.wah.set((value*(WAH_MAX.ln() - WAH_MIN.ln()) + WAH_MIN.ln()).exp()),
            _ => ()
        }
    }

    fn change_preset(&self, preset: i32) {}

    fn get_preset_num(&self) -> i32 {
        0
    }

    fn set_preset_name(&self, name: String) {}

    fn get_preset_name(&self, preset: i32) -> String {
        "".to_string()
    }

    fn can_be_automated(&self, index: i32) -> bool {
        index < 1
    }

    fn get_preset_data(&self) -> Vec<u8> {
        [
            self.wah.get().to_le_bytes().to_vec()
        ].concat()
    }

    fn get_bank_data(&self) -> Vec<u8> {
        [
            self.wah.get().to_le_bytes().to_vec()
        ].concat()
    }

    fn load_preset_data(&self, data: &[u8])
    {
        self.wah.set(f32::from_le_bytes(*data[(data.len() - 4)..].split_array_ref().0));
    }

    fn load_bank_data(&self, data: &[u8])
    {
        self.wah.set(f32::from_le_bytes(*data[(data.len() - 4)..].split_array_ref().0));
    }
}