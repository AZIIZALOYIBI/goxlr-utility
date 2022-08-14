use std::collections::HashMap;
use std::io::Write;
use std::os::raw::c_float;

use enum_map::EnumMap;
use strum::{EnumIter, EnumProperty, IntoEnumIterator};
use xml::attribute::OwnedAttribute;
use xml::writer::events::StartElementBuilder;
use xml::writer::XmlEvent as XmlWriterEvent;
use xml::EventWriter;

use anyhow::{anyhow, Result};

use crate::components::colours::ColourMap;
use crate::components::megaphone::MegaphoneStyle::Megaphone;
use crate::Preset;
use crate::Preset::{Preset1, Preset2, Preset3, Preset4, Preset5, Preset6};

#[derive(thiserror::Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum ParseError {
    #[error("Expected int: {0}")]
    ExpectedInt(#[from] std::num::ParseIntError),

    #[error("Expected float: {0}")]
    ExpectedFloat(#[from] std::num::ParseFloatError),

    #[error("Expected enum: {0}")]
    ExpectedEnum(#[from] strum::ParseError),

    #[error("Invalid colours: {0}")]
    InvalidColours(#[from] crate::components::colours::ParseError),
}

/**
 * This is relatively static, main tag contains standard colour mapping, subtags contain various
 * presets, we'll use an EnumMap to define the 'presets' as they'll be useful for the other various
 * 'types' of presets (encoders and effects).
 */
#[derive(Debug)]
pub struct MegaphoneEffectBase {
    colour_map: ColourMap,
    preset_map: EnumMap<Preset, MegaphoneEffect>,
}

impl MegaphoneEffectBase {
    pub fn new(element_name: String) -> Self {
        let colour_map = element_name;
        Self {
            colour_map: ColourMap::new(colour_map),
            preset_map: EnumMap::default(),
        }
    }

    pub fn parse_megaphone_root(&mut self, attributes: &[OwnedAttribute]) -> Result<()> {
        for attr in attributes {
            if !self.colour_map.read_colours(attr)? {
                println!("[megaphoneEffect] Unparsed Attribute: {}", attr.name);
            }
        }

        Ok(())
    }

    pub fn parse_megaphone_preset(&mut self, id: u8, attributes: &[OwnedAttribute]) -> Result<()> {
        let mut preset = MegaphoneEffect::new();
        for attr in attributes {
            if attr.name.local_name == "megaphoneEffectstate" {
                preset.state = matches!(attr.value.as_str(), "1");
                continue;
            }
            if attr.name.local_name == "MEGAPHONE_STYLE" {
                for style in MegaphoneStyle::iter() {
                    if style.get_str("uiIndex").unwrap() == attr.value {
                        preset.style = style;
                        break;
                    }
                }
                continue;
            }

            /*
             * As batshit as the below code seems, in some cases the Windows UI will spit out
             * values as floats, despite those floats representing whole numbers (eg 5.00000), so
             * for all cases here, we're going to read the numbers in as floats, then convert them
             * across to their correct type.
             */

            if attr.name.local_name == "TRANS_DIST_AMT" {
                preset.trans_dist_amt = attr.value.parse::<c_float>()? as u8;
                continue;
            }
            if attr.name.local_name == "TRANS_HP" {
                preset.trans_hp = attr.value.parse::<c_float>()? as u8;
                continue;
            }
            if attr.name.local_name == "TRANS_LP" {
                preset.trans_lp = attr.value.parse::<c_float>()? as u8;
                continue;
            }
            if attr.name.local_name == "TRANS_PREGAIN" {
                preset.trans_pregain = attr.value.parse::<c_float>()? as u8;
                continue;
            }
            if attr.name.local_name == "TRANS_POSTGAIN" {
                preset.trans_postgain = attr.value.parse::<c_float>()? as i8;
                continue;
            }
            if attr.name.local_name == "TRANS_DIST_TYPE" {
                preset.trans_dist_type = attr.value.parse::<c_float>()? as u8;
                continue;
            }
            if attr.name.local_name == "TRANS_PRESENCE_GAIN" {
                preset.trans_presence_gain = attr.value.parse::<c_float>()? as u8;
                continue;
            }
            if attr.name.local_name == "TRANS_PRESENCE_FC" {
                preset.trans_presence_fc = attr.value.parse::<c_float>()? as u8;
                continue;
            }
            if attr.name.local_name == "TRANS_PRESENCE_BW" {
                preset.trans_presence_bw = attr.value.parse::<c_float>()? as u8;
                continue;
            }
            if attr.name.local_name == "TRANS_BEATBOX_ENABLE" {
                preset.trans_beatbox_enabled = attr.value != "0";
                continue;
            }
            if attr.name.local_name == "TRANS_FILTER_CONTROL" {
                preset.trans_filter_control = attr.value.parse::<c_float>()? as u8;
                continue;
            }
            if attr.name.local_name == "TRANS_FILTER" {
                preset.trans_filter = attr.value.parse::<c_float>()? as u8;
                continue;
            }
            if attr.name.local_name == "TRANS_DRIVE_POT_GAIN_COMP_MID" {
                preset.trans_drive_pot_gain_comp_mid = attr.value.parse::<c_float>()? as u8;
                continue;
            }
            if attr.name.local_name == "TRANS_DRIVE_POT_GAIN_COMP_MAX" {
                preset.trans_drive_pot_gain_comp_max = attr.value.parse::<c_float>()? as u8;
                continue;
            }
            println!(
                "[MegaphoneEffect] Unparsed Child Attribute: {}",
                &attr.name.local_name
            );
        }

        // Ok, we should be able to store this now..
        if id == 1 {
            self.preset_map[Preset1] = preset;
        } else if id == 2 {
            self.preset_map[Preset2] = preset;
        } else if id == 3 {
            self.preset_map[Preset3] = preset;
        } else if id == 4 {
            self.preset_map[Preset4] = preset;
        } else if id == 5 {
            self.preset_map[Preset5] = preset;
        } else if id == 6 {
            self.preset_map[Preset6] = preset;
        }

        Ok(())
    }

    pub fn write_megaphone<W: Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<()> {
        let mut element: StartElementBuilder = XmlWriterEvent::start_element("megaphoneEffect");

        let mut attributes: HashMap<String, String> = HashMap::default();
        self.colour_map.write_colours(&mut attributes);

        // Write out the attributes etc for this element, but don't close it yet..
        for (key, value) in &attributes {
            element = element.attr(key.as_str(), value.as_str());
        }

        writer.write(element)?;

        // Because all of these are seemingly 'guaranteed' to exist, we can straight dump..
        for (key, value) in &self.preset_map {
            let mut sub_attributes: HashMap<String, String> = HashMap::default();

            let tag_name = format!("megaphoneEffect{}", key.get_str("tagSuffix").unwrap());
            let mut sub_element: StartElementBuilder =
                XmlWriterEvent::start_element(tag_name.as_str());

            sub_attributes.insert(
                "megaphoneEffectstate".to_string(),
                if value.state {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            );
            sub_attributes.insert(
                "MEGAPHONE_STYLE".to_string(),
                value.style.get_str("uiIndex").unwrap().to_string(),
            );
            sub_attributes.insert(
                "TRANS_DIST_AMT".to_string(),
                format!("{}", value.trans_dist_amt),
            );
            sub_attributes.insert("TRANS_HP".to_string(), format!("{}", value.trans_hp));
            sub_attributes.insert("TRANS_LP".to_string(), format!("{}", value.trans_lp));
            sub_attributes.insert(
                "TRANS_PREGAIN".to_string(),
                format!("{}", value.trans_pregain),
            );
            sub_attributes.insert(
                "TRANS_POSTGAIN".to_string(),
                format!("{}", value.trans_postgain),
            );
            sub_attributes.insert(
                "TRANS_DIST_TYPE".to_string(),
                format!("{}", value.trans_dist_type),
            );
            sub_attributes.insert(
                "TRANS_PRESENCE_GAIN".to_string(),
                format!("{}", value.trans_presence_gain),
            );
            sub_attributes.insert(
                "TRANS_PRESENCE_FC".to_string(),
                format!("{}", value.trans_presence_fc),
            );
            sub_attributes.insert(
                "TRANS_PRESENCE_BW".to_string(),
                format!("{}", value.trans_presence_bw),
            );
            sub_attributes.insert(
                "TRANS_BEATBOX_ENABLE".to_string(),
                if value.trans_beatbox_enabled {
                    "1".to_string()
                } else {
                    "0".to_string()
                },
            );
            sub_attributes.insert(
                "TRANS_FILTER_CONTROL".to_string(),
                format!("{}", value.trans_filter_control),
            );
            sub_attributes.insert(
                "TRANS_FILTER".to_string(),
                format!("{}", value.trans_filter),
            );
            sub_attributes.insert(
                "TRANS_DRIVE_POT_GAIN_COMP_MID".to_string(),
                format!("{}", value.trans_drive_pot_gain_comp_mid),
            );
            sub_attributes.insert(
                "TRANS_DRIVE_POT_GAIN_COMP_MAX".to_string(),
                format!("{}", value.trans_drive_pot_gain_comp_max),
            );

            for (key, value) in &sub_attributes {
                sub_element = sub_element.attr(key.as_str(), value.as_str());
            }

            writer.write(sub_element)?;
            writer.write(XmlWriterEvent::end_element())?;
        }

        // Finally, close the 'main' tag.
        writer.write(XmlWriterEvent::end_element())?;
        Ok(())
    }

    pub fn colour_map(&self) -> &ColourMap {
        &self.colour_map
    }

    pub fn colour_map_mut(&mut self) -> &mut ColourMap {
        &mut self.colour_map
    }

    pub fn get_preset(&self, preset: Preset) -> &MegaphoneEffect {
        &self.preset_map[preset]
    }

    pub fn get_preset_mut(&mut self, preset: Preset) -> &mut MegaphoneEffect {
        &mut self.preset_map[preset]
    }
}

/**
 * Couple of interesting points, firstly, the UI only has 3 options with regards to the
 * megaphone configuration, Style, 'Amount', and 'Post Gain', yet these three options
 * ultimately translate into *MANY* different settings, so some investigation as to how
 * and why these map will be necessary. I'm currently assuming that each 'style' is backed
 * by several values, but still need to work out the mapping.
 *
 */
#[derive(Debug, Default)]
pub struct MegaphoneEffect {
    // State here determines if the megaphone is on or off when this preset is loaded.
    state: bool,

    style: MegaphoneStyle,
    trans_dist_amt: u8,
    trans_hp: u8,
    trans_lp: u8,
    trans_pregain: u8,
    trans_postgain: i8,
    trans_dist_type: u8,
    trans_presence_gain: u8,
    trans_presence_fc: u8,
    trans_presence_bw: u8,
    trans_beatbox_enabled: bool,
    trans_filter_control: u8,
    trans_filter: u8,
    trans_drive_pot_gain_comp_mid: u8,
    trans_drive_pot_gain_comp_max: u8,
}

impl MegaphoneEffect {
    pub fn new() -> Self {
        Self {
            state: false,
            style: Megaphone,
            trans_dist_amt: 0,
            trans_hp: 0,
            trans_lp: 0,
            trans_pregain: 0,
            trans_postgain: 0,
            trans_dist_type: 0,
            trans_presence_gain: 0,
            trans_presence_fc: 0,
            trans_presence_bw: 0,
            trans_beatbox_enabled: false,
            trans_filter_control: 0,
            trans_filter: 0,
            trans_drive_pot_gain_comp_mid: 0,
            trans_drive_pot_gain_comp_max: 0,
        }
    }

    pub fn state(&self) -> bool {
        self.state
    }
    pub fn set_state(&mut self, state: bool) {
        self.state = state;
    }

    pub fn style(&self) -> &MegaphoneStyle {
        &self.style
    }
    pub fn set_style(&mut self, style: MegaphoneStyle) -> Result<()> {
        self.style = style;

        let preset = MegaphonePreset::get_preset(style);
        self.set_trans_dist_amt(preset.trans_dist_amt)?;
        self.set_trans_hp(preset.trans_hp);
        self.set_trans_lp(preset.trans_lp);
        self.set_trans_pregain(preset.trans_pregain);
        self.set_trans_postgain(preset.trans_postgain)?;
        self.set_trans_dist_type(preset.trans_dist_type);
        self.set_trans_presence_gain(preset.trans_presence_gain);
        self.set_trans_presence_fc(preset.trans_presence_fc);
        self.set_trans_presence_bw(preset.trans_presence_bw);
        self.set_trans_beatbox_enabled(preset.trans_beatbox_enabled);
        self.set_trans_filter_control(preset.trans_filter_control);
        self.set_trans_filter(preset.trans_filter);
        self.set_trans_drive_pot_gain_comp_mid(preset.trans_drive_pot_gain_comp_mid);
        self.set_trans_drive_pot_gain_comp_max(preset.trans_drive_pot_gain_comp_max);

        Ok(())
    }

    pub fn trans_dist_amt(&self) -> u8 {
        self.trans_dist_amt
    }
    pub fn set_trans_dist_amt(&mut self, value: u8) -> Result<()> {
        if value > 100 {
            return Err(anyhow!("Amount should be a percentage"));
        }
        self.trans_dist_amt = value;
        Ok(())
    }

    pub fn trans_hp(&self) -> u8 {
        self.trans_hp
    }
    fn set_trans_hp(&mut self, trans_hp: u8) {
        self.trans_hp = trans_hp;
    }

    pub fn trans_lp(&self) -> u8 {
        self.trans_lp
    }
    fn set_trans_lp(&mut self, trans_lp: u8) {
        self.trans_lp = trans_lp;
    }

    pub fn trans_pregain(&self) -> u8 {
        self.trans_pregain
    }
    fn set_trans_pregain(&mut self, trans_pregain: u8) {
        self.trans_pregain = trans_pregain;
    }

    pub fn trans_postgain(&self) -> i8 {
        self.trans_postgain
    }
    pub fn set_trans_postgain(&mut self, trans_postgain: i8) -> Result<()> {
        if !(-20..=20).contains(&trans_postgain) {
            return Err(anyhow!("Post Gain should be between -20 and 20"));
        }
        self.trans_postgain = trans_postgain;
        Ok(())
    }

    pub fn trans_dist_type(&self) -> u8 {
        self.trans_dist_type
    }
    fn set_trans_dist_type(&mut self, trans_dist_type: u8) {
        self.trans_dist_type = trans_dist_type;
    }

    pub fn trans_presence_gain(&self) -> u8 {
        self.trans_presence_gain
    }
    fn set_trans_presence_gain(&mut self, trans_presence_gain: u8) {
        self.trans_presence_gain = trans_presence_gain;
    }

    pub fn trans_presence_fc(&self) -> u8 {
        self.trans_presence_fc
    }
    fn set_trans_presence_fc(&mut self, trans_presence_fc: u8) {
        self.trans_presence_fc = trans_presence_fc;
    }

    pub fn trans_presence_bw(&self) -> u8 {
        self.trans_presence_bw
    }
    fn set_trans_presence_bw(&mut self, trans_presence_bw: u8) {
        self.trans_presence_bw = trans_presence_bw;
    }

    pub fn trans_beatbox_enabled(&self) -> bool {
        self.trans_beatbox_enabled
    }
    fn set_trans_beatbox_enabled(&mut self, trans_beatbox_enabled: bool) {
        self.trans_beatbox_enabled = trans_beatbox_enabled;
    }

    pub fn trans_filter_control(&self) -> u8 {
        self.trans_filter_control
    }
    fn set_trans_filter_control(&mut self, trans_filter_control: u8) {
        self.trans_filter_control = trans_filter_control;
    }

    pub fn trans_filter(&self) -> u8 {
        self.trans_filter
    }
    fn set_trans_filter(&mut self, trans_filter: u8) {
        self.trans_filter = trans_filter;
    }

    pub fn trans_drive_pot_gain_comp_mid(&self) -> u8 {
        self.trans_drive_pot_gain_comp_mid
    }
    fn set_trans_drive_pot_gain_comp_mid(&mut self, trans_drive_pot_gain_comp_mid: u8) {
        self.trans_drive_pot_gain_comp_mid = trans_drive_pot_gain_comp_mid;
    }

    pub fn trans_drive_pot_gain_comp_max(&self) -> u8 {
        self.trans_drive_pot_gain_comp_max
    }
    fn set_trans_drive_pot_gain_comp_max(&mut self, trans_drive_pot_gain_comp_max: u8) {
        self.trans_drive_pot_gain_comp_max = trans_drive_pot_gain_comp_max;
    }
}

#[derive(Debug, EnumIter, EnumProperty, Copy, Clone)]
pub enum MegaphoneStyle {
    #[strum(props(uiIndex = "0"))]
    Megaphone,

    #[strum(props(uiIndex = "1"))]
    Radio,

    #[strum(props(uiIndex = "2"))]
    OnThePhone,

    #[strum(props(uiIndex = "3"))]
    Overdrive,

    #[strum(props(uiIndex = "4"))]
    BuzzCutt,

    #[strum(props(uiIndex = "5"))]
    Tweed,
}

impl Default for MegaphoneStyle {
    fn default() -> Self {
        Megaphone
    }
}

struct MegaphonePreset {
    trans_dist_amt: u8,
    trans_hp: u8,
    trans_lp: u8,
    trans_pregain: u8,
    trans_postgain: i8,
    trans_dist_type: u8,
    trans_presence_gain: u8,
    trans_presence_fc: u8,
    trans_presence_bw: u8,
    trans_beatbox_enabled: bool,
    trans_filter_control: u8,
    trans_filter: u8,
    trans_drive_pot_gain_comp_mid: u8,
    trans_drive_pot_gain_comp_max: u8,
}

impl MegaphonePreset {
    fn get_preset(style: MegaphoneStyle) -> MegaphonePreset {
        match style {
            Megaphone => MegaphonePreset {
                trans_dist_amt: 0,
                trans_hp: 120,
                trans_lp: 200,
                trans_pregain: 0,
                trans_postgain: 2,
                trans_dist_type: 6,
                trans_presence_gain: 8,
                trans_presence_fc: 135,
                trans_presence_bw: 7,
                trans_beatbox_enabled: false,
                trans_filter_control: 2,
                trans_filter: 59,
                trans_drive_pot_gain_comp_mid: 0,
                trans_drive_pot_gain_comp_max: 0,
            },
            MegaphoneStyle::Radio => MegaphonePreset {
                trans_dist_amt: 30,
                trans_hp: 110,
                trans_lp: 190,
                trans_pregain: 0,
                trans_postgain: 2,
                trans_dist_type: 4,
                trans_presence_gain: 7,
                trans_presence_fc: 160,
                trans_presence_bw: 5,
                trans_beatbox_enabled: false,
                trans_filter_control: 1,
                trans_filter: 59,
                trans_drive_pot_gain_comp_mid: 0,
                trans_drive_pot_gain_comp_max: 5,
            },
            MegaphoneStyle::OnThePhone => MegaphonePreset {
                trans_dist_amt: 50,
                trans_hp: 50,
                trans_lp: 238,
                trans_pregain: 0,
                trans_postgain: 0,
                trans_dist_type: 12,
                trans_presence_gain: 10,
                trans_presence_fc: 160,
                trans_presence_bw: 5,
                trans_beatbox_enabled: false,
                trans_filter_control: 3,
                trans_filter: 0,
                trans_drive_pot_gain_comp_mid: 0,
                trans_drive_pot_gain_comp_max: 0,
            },
            MegaphoneStyle::Overdrive => MegaphonePreset {
                trans_dist_amt: 50,
                trans_hp: 50,
                trans_lp: 238,
                trans_pregain: 0,
                trans_postgain: 2,
                trans_dist_type: 1,
                trans_presence_gain: 0,
                trans_presence_fc: 168,
                trans_presence_bw: 8,
                trans_beatbox_enabled: false,
                trans_filter_control: 1,
                trans_filter: 100,
                trans_drive_pot_gain_comp_mid: 1,
                trans_drive_pot_gain_comp_max: 25,
            },
            MegaphoneStyle::BuzzCutt => MegaphonePreset {
                trans_dist_amt: 50,
                trans_hp: 50,
                trans_lp: 238,
                trans_pregain: 0,
                trans_postgain: 2,
                trans_dist_type: 9,
                trans_presence_gain: 5,
                trans_presence_fc: 174,
                trans_presence_bw: 4,
                trans_beatbox_enabled: false,
                trans_filter_control: 3,
                trans_filter: 100,
                trans_drive_pot_gain_comp_mid: 1,
                trans_drive_pot_gain_comp_max: 8,
            },
            MegaphoneStyle::Tweed => MegaphonePreset {
                trans_dist_amt: 20,
                trans_hp: 78,
                trans_lp: 192,
                trans_pregain: 10,
                trans_postgain: 2,
                trans_dist_type: 13,
                trans_presence_gain: 0,
                trans_presence_fc: 168,
                trans_presence_bw: 8,
                trans_beatbox_enabled: false,
                trans_filter_control: 3,
                trans_filter: 59,
                trans_drive_pot_gain_comp_mid: 3,
                trans_drive_pot_gain_comp_max: 4,
            },
        }
    }
}
