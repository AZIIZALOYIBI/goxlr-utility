use std::collections::HashMap;
use std::os::raw::c_float;
use std::str::FromStr;
use xml::attribute::OwnedAttribute;

#[derive(thiserror::Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum ParseError {
    #[error("Expected int: {0}")]
    ExpectedInt(#[from] std::num::ParseIntError),

    #[error("Expected float: {0}")]
    ExpectedFloat(#[from] std::num::ParseFloatError),
}

// The EQ has a crap load of values (20 total), we could consider splitting
// them into Gain and Freq to keep stuff tidy?
#[derive(Debug)]
pub struct Equalizer {
    eq_31h_gain: i8,
    eq_63h_gain: i8,
    eq_125h_gain: i8,
    eq_250h_gain: i8,
    eq_500h_gain: i8,
    eq_1k_gain: i8,
    eq_2k_gain: i8,
    eq_4k_gain: i8,
    eq_8k_gain: i8,
    eq_16k_gain: i8,

    eq_31h_freq: f32,
    eq_63h_freq: f32,
    eq_125h_freq: f32,
    eq_250h_freq: f32,
    eq_500h_freq: f32,
    eq_1k_freq: f32,
    eq_2k_freq: f32,
    eq_4k_freq: f32,
    eq_8k_freq: f32,
    eq_16k_freq: f32,
}

impl Default for Equalizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Equalizer {
    pub fn new() -> Self {
        Self {
            eq_31h_gain: 0,
            eq_63h_gain: 0,
            eq_125h_gain: 0,
            eq_250h_gain: 0,
            eq_500h_gain: 0,
            eq_1k_gain: 0,
            eq_2k_gain: 0,
            eq_4k_gain: 0,
            eq_8k_gain: 0,
            eq_16k_gain: 0,
            eq_31h_freq: 31.5,
            eq_63h_freq: 63.0,
            eq_125h_freq: 125.0,
            eq_250h_freq: 250.0,
            eq_500h_freq: 500.0,
            eq_1k_freq: 1000.0,
            eq_2k_freq: 2000.0,
            eq_4k_freq: 4000.0,
            eq_8k_freq: 8000.0,
            eq_16k_freq: 16000.0,
        }
    }

    pub fn parse_equaliser(&mut self, attributes: &[OwnedAttribute]) -> Result<(), ParseError> {
        for attr in attributes {
            if attr.name.local_name == "MIC_EQ_31.5HZ_GAIN" {
                self.eq_31h_gain = attr.value.parse::<c_float>()? as i8
            }

            if attr.name.local_name == "MIC_EQ_63HZ_GAIN" {
                self.eq_63h_gain = attr.value.parse::<c_float>()? as i8
            }

            if attr.name.local_name == "MIC_EQ_125HZ_GAIN" {
                self.eq_125h_gain = attr.value.parse::<c_float>()? as i8
            }

            if attr.name.local_name == "MIC_EQ_250HZ_GAIN" {
                self.eq_250h_gain = attr.value.parse::<c_float>()? as i8
            }

            if attr.name.local_name == "MIC_EQ_500HZ_GAIN" {
                self.eq_500h_gain = attr.value.parse::<c_float>()? as i8
            }

            if attr.name.local_name == "MIC_EQ_1KHZ_GAIN" {
                self.eq_1k_gain = attr.value.parse::<c_float>()? as i8
            }

            if attr.name.local_name == "MIC_EQ_2KHZ_GAIN" {
                self.eq_2k_gain = attr.value.parse::<c_float>()? as i8
            }

            if attr.name.local_name == "MIC_EQ_4KHZ_GAIN" {
                self.eq_4k_gain = attr.value.parse::<c_float>()? as i8
            }

            if attr.name.local_name == "MIC_EQ_8KHZ_GAIN" {
                self.eq_8k_gain = attr.value.parse::<c_float>()? as i8
            }

            if attr.name.local_name == "MIC_EQ_16KHZ_GAIN" {
                self.eq_16k_gain = attr.value.parse::<c_float>()? as i8
            }

            if attr.name.local_name == "MIC_EQ_31.5HZ_F" {
                self.eq_31h_freq = f32::from_str(attr.value.as_str())?;
            }

            if attr.name.local_name == "MIC_EQ_63HZ_F" {
                self.eq_63h_freq = f32::from_str(attr.value.as_str())?;
            }

            if attr.name.local_name == "MIC_EQ_125HZ_F" {
                self.eq_125h_freq = f32::from_str(attr.value.as_str())?;
            }

            if attr.name.local_name == "MIC_EQ_250HZ_F" {
                self.eq_250h_freq = f32::from_str(attr.value.as_str())?;
            }

            if attr.name.local_name == "MIC_EQ_500HZ_F" {
                self.eq_500h_freq = f32::from_str(attr.value.as_str())?;
            }

            if attr.name.local_name == "MIC_EQ_1KHZ_F" {
                self.eq_1k_freq = f32::from_str(attr.value.as_str())?;
            }

            if attr.name.local_name == "MIC_EQ_2KHZ_F" {
                self.eq_2k_freq = f32::from_str(attr.value.as_str())?;
            }

            if attr.name.local_name == "MIC_EQ_4KHZ_F" {
                self.eq_4k_freq = f32::from_str(attr.value.as_str())?;
            }

            if attr.name.local_name == "MIC_EQ_8KHZ_F" {
                self.eq_8k_freq = f32::from_str(attr.value.as_str())?;
            }

            if attr.name.local_name == "MIC_EQ_16KHZ_F" {
                self.eq_16k_freq = f32::from_str(attr.value.as_str())?;
            }
        }

        Ok(())
    }

    pub fn write_equaliser(&self, attributes: &mut HashMap<String, String>) {
        attributes.insert(
            "MIC_EQ_31.5HZ_GAIN".to_string(),
            format!("{}", self.eq_31h_gain),
        );
        attributes.insert(
            "MIC_EQ_63HZ_GAIN".to_string(),
            format!("{}", self.eq_63h_gain),
        );
        attributes.insert(
            "MIC_EQ_125HZ_GAIN".to_string(),
            format!("{}", self.eq_125h_gain),
        );
        attributes.insert(
            "MIC_EQ_250HZ_GAIN".to_string(),
            format!("{}", self.eq_250h_gain),
        );
        attributes.insert(
            "MIC_EQ_500HZ_GAIN".to_string(),
            format!("{}", self.eq_500h_gain),
        );
        attributes.insert(
            "MIC_EQ_1KHZ_GAIN".to_string(),
            format!("{}", self.eq_1k_gain),
        );
        attributes.insert(
            "MIC_EQ_2KHZ_GAIN".to_string(),
            format!("{}", self.eq_2k_gain),
        );
        attributes.insert(
            "MIC_EQ_4KHZ_GAIN".to_string(),
            format!("{}", self.eq_4k_gain),
        );
        attributes.insert(
            "MIC_EQ_8KHZ_GAIN".to_string(),
            format!("{}", self.eq_8k_gain),
        );
        attributes.insert(
            "MIC_EQ_16KHZ_GAIN".to_string(),
            format!("{}", self.eq_16k_gain),
        );

        attributes.insert(
            "MIC_EQ_31.5HZ_F".to_string(),
            format!("{}", self.eq_31h_freq),
        );
        attributes.insert("MIC_EQ_63HZ_F".to_string(), format!("{}", self.eq_63h_freq));
        attributes.insert(
            "MIC_EQ_125HZ_F".to_string(),
            format!("{}", self.eq_125h_freq),
        );
        attributes.insert(
            "MIC_EQ_250HZ_F".to_string(),
            format!("{}", self.eq_250h_freq),
        );
        attributes.insert(
            "MIC_EQ_500HZ_F".to_string(),
            format!("{}", self.eq_500h_freq),
        );
        attributes.insert("MIC_EQ_1KHZ_F".to_string(), format!("{}", self.eq_1k_freq));
        attributes.insert("MIC_EQ_2KHZ_F".to_string(), format!("{}", self.eq_2k_freq));
        attributes.insert("MIC_EQ_4KHZ_F".to_string(), format!("{}", self.eq_4k_freq));
        attributes.insert("MIC_EQ_8KHZ_F".to_string(), format!("{}", self.eq_8k_freq));
        attributes.insert(
            "MIC_EQ_16KHZ_F".to_string(),
            format!("{}", self.eq_16k_freq),
        );
    }

    pub fn eq_31h_gain(&self) -> i8 {
        self.eq_31h_gain
    }
    pub fn eq_63h_gain(&self) -> i8 {
        self.eq_63h_gain
    }
    pub fn eq_125h_gain(&self) -> i8 {
        self.eq_125h_gain
    }
    pub fn eq_250h_gain(&self) -> i8 {
        self.eq_250h_gain
    }
    pub fn eq_500h_gain(&self) -> i8 {
        self.eq_500h_gain
    }
    pub fn eq_1k_gain(&self) -> i8 {
        self.eq_1k_gain
    }
    pub fn eq_2k_gain(&self) -> i8 {
        self.eq_2k_gain
    }
    pub fn eq_4k_gain(&self) -> i8 {
        self.eq_4k_gain
    }
    pub fn eq_8k_gain(&self) -> i8 {
        self.eq_8k_gain
    }
    pub fn eq_16k_gain(&self) -> i8 {
        self.eq_16k_gain
    }
    pub fn eq_31h_freq(&self) -> f32 {
        self.eq_31h_freq
    }
    pub fn eq_63h_freq(&self) -> f32 {
        self.eq_63h_freq
    }
    pub fn eq_125h_freq(&self) -> f32 {
        self.eq_125h_freq
    }
    pub fn eq_250h_freq(&self) -> f32 {
        self.eq_250h_freq
    }
    pub fn eq_500h_freq(&self) -> f32 {
        self.eq_500h_freq
    }
    pub fn eq_1k_freq(&self) -> f32 {
        self.eq_1k_freq
    }
    pub fn eq_2k_freq(&self) -> f32 {
        self.eq_2k_freq
    }
    pub fn eq_4k_freq(&self) -> f32 {
        self.eq_4k_freq
    }
    pub fn eq_8k_freq(&self) -> f32 {
        self.eq_8k_freq
    }
    pub fn eq_16k_freq(&self) -> f32 {
        self.eq_16k_freq
    }

    pub fn eq_31h_freq_as_goxlr(&self) -> i32 {
        self.freq_value(self.eq_31h_freq)
    }
    pub fn eq_63h_freq_as_goxlr(&self) -> i32 {
        self.freq_value(self.eq_63h_freq)
    }
    pub fn eq_125h_freq_as_goxlr(&self) -> i32 {
        self.freq_value(self.eq_125h_freq)
    }
    pub fn eq_250h_freq_as_goxlr(&self) -> i32 {
        self.freq_value(self.eq_250h_freq)
    }
    pub fn eq_500h_freq_as_goxlr(&self) -> i32 {
        self.freq_value(self.eq_500h_freq)
    }
    pub fn eq_1k_freq_as_goxlr(&self) -> i32 {
        self.freq_value(self.eq_1k_freq)
    }
    pub fn eq_2k_freq_as_goxlr(&self) -> i32 {
        self.freq_value(self.eq_2k_freq)
    }
    pub fn eq_4k_freq_as_goxlr(&self) -> i32 {
        self.freq_value(self.eq_4k_freq)
    }
    pub fn eq_8k_freq_as_goxlr(&self) -> i32 {
        self.freq_value(self.eq_8k_freq)
    }
    pub fn eq_16k_freq_as_goxlr(&self) -> i32 {
        self.freq_value(self.eq_16k_freq)
    }

    pub fn set_eq_31h_gain(&mut self, eq_31h_gain: i8) {
        self.eq_31h_gain = eq_31h_gain;
    }
    pub fn set_eq_63h_gain(&mut self, eq_63h_gain: i8) {
        self.eq_63h_gain = eq_63h_gain;
    }
    pub fn set_eq_125h_gain(&mut self, eq_125h_gain: i8) {
        self.eq_125h_gain = eq_125h_gain;
    }
    pub fn set_eq_250h_gain(&mut self, eq_250h_gain: i8) {
        self.eq_250h_gain = eq_250h_gain;
    }
    pub fn set_eq_500h_gain(&mut self, eq_500h_gain: i8) {
        self.eq_500h_gain = eq_500h_gain;
    }
    pub fn set_eq_1k_gain(&mut self, eq_1k_gain: i8) {
        self.eq_1k_gain = eq_1k_gain;
    }
    pub fn set_eq_2k_gain(&mut self, eq_2k_gain: i8) {
        self.eq_2k_gain = eq_2k_gain;
    }
    pub fn set_eq_4k_gain(&mut self, eq_4k_gain: i8) {
        self.eq_4k_gain = eq_4k_gain;
    }
    pub fn set_eq_8k_gain(&mut self, eq_8k_gain: i8) {
        self.eq_8k_gain = eq_8k_gain;
    }
    pub fn set_eq_16k_gain(&mut self, eq_16k_gain: i8) {
        self.eq_16k_gain = eq_16k_gain;
    }
    pub fn set_eq_31h_freq(&mut self, eq_31h_freq: f32) {
        self.eq_31h_freq = eq_31h_freq;
    }
    pub fn set_eq_63h_freq(&mut self, eq_63h_freq: f32) {
        self.eq_63h_freq = eq_63h_freq;
    }
    pub fn set_eq_125h_freq(&mut self, eq_125h_freq: f32) {
        self.eq_125h_freq = eq_125h_freq;
    }
    pub fn set_eq_250h_freq(&mut self, eq_250h_freq: f32) {
        self.eq_250h_freq = eq_250h_freq;
    }
    pub fn set_eq_500h_freq(&mut self, eq_500h_freq: f32) {
        self.eq_500h_freq = eq_500h_freq;
    }
    pub fn set_eq_1k_freq(&mut self, eq_1k_freq: f32) {
        self.eq_1k_freq = eq_1k_freq;
    }
    pub fn set_eq_2k_freq(&mut self, eq_2k_freq: f32) {
        self.eq_2k_freq = eq_2k_freq;
    }
    pub fn set_eq_4k_freq(&mut self, eq_4k_freq: f32) {
        self.eq_4k_freq = eq_4k_freq;
    }
    pub fn set_eq_8k_freq(&mut self, eq_8k_freq: f32) {
        self.eq_8k_freq = eq_8k_freq;
    }
    pub fn set_eq_16k_freq(&mut self, eq_16k_freq: f32) {
        self.eq_16k_freq = eq_16k_freq;
    }

    fn freq_value(&self, freq: f32) -> i32 {
        ((24.0 * (freq / 20.0).log2()) as f32).round() as i32
    }
}
