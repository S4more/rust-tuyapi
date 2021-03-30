use serde_json::json;
use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    WHITE,
    COLOUR,
    SCENE,
    MUSIC
}

// Assuming that all RGB bulbs follow the same pattern.
// I'm aware that some start at 1 and some at 20.
pub struct DpsStruct {
    pub offset: u8,
    pub switch: Option<bool>,
    pub mode: Option<Mode>,
    pub bright: Option<u16>,
    pub color_temp: Option<u16>,
    pub color: Option<String>, // h: 0-360, s: 0-1000, v:0-1000
    pub scene: Option<String>
}

impl Default for DpsStruct {
    fn default() -> Self {
        DpsStruct {
            offset: 20,
            switch: None,
            mode: None,
            bright: None,
            color_temp: None,
            color: None,
            scene: None
        }
    }
}

impl crate::Dps for DpsStruct {
    fn dps(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();

        if let Some(v) = self.switch{
            map.insert(self.offset.to_string(), json!(v));
        }

        if let Some(v) = &self.mode {
            map.insert( (self.offset + 1).to_string(), json!(v));
        }

        if let Some(v) = self.bright {
            map.insert( (self.offset + 2).to_string(), json!(v));
        }

        if let Some(v) = self.color_temp {
            map.insert( (self.offset + 3).to_string(), json!(v));
        }

        if let Some(v) = &self.color {
            map.insert( (self.offset + 4).to_string(), json!(v));
        }

        if let Some(v) = &self.scene {
            map.insert( (self.offset + 5).to_string(), json!(v));
        }


        map
    }
}
