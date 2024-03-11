use crate::com::*;

use super::*;

use serde_json::{Map, Value};
use std::convert::From;

const MAX_FREQUENCY: f64 = 192000.0;
const MAX_STATE_SIZE: u64 = 20 * 1024 * 1024;
const MAX_STACK_SIZE: u64 = 5 * 1024 * 1024;
const EVENT_BUFFER_SIZE: u32 = 32;
const MAX_BLOCK_SIZE: u32 = 1024;
const MAX_POOL_SIZE: u32 = 50 * 1024 * 1024;

pub struct BuildSettings {
    settings: serde_json::Value,
}

impl BuildSettings {
    const MAX_FREQUENCY_MEMBER: &str = "maxFrequency";
    const FREQUENCY_MEMBER: &str = "frequency";
    const MAX_BLOCK_SIZE_MEMBER: &str = "maxBlockSize";
    const MAX_STATE_SIZE_MEMBER: &str = "maxStateSize";
    const MAX_STACK_SIZE_MEMBER: &str = "maxStackSize";
    const EVENT_BUFFER_SIZE_MEMBER: &str = "eventBufferSize";
    const MAX_POOL_SIZE_MEMBER: &str = "maxAllocPoolSize";
    const OPTIMIZATION_LEVEL_MEMBER: &str = "optimisationLevel";
    const SESSION_ID_MEMBER: &str = "sessionID";
    const IGNORE_WARNING_MEMBER: &str = "ignoreWarnings";
    const DEBUG_MEMBER: &str = "debug";
    const MAIN_PROCESSOR_MEMBER: &str = "mainProcessor";

    pub fn get_max_frequency(&self) -> f64 {
        match self.settings.get(Self::MAX_FREQUENCY_MEMBER) {
            Some(value) => value
                .as_f64()
                .unwrap_or(MAX_FREQUENCY)
                .clamp(1.0, 1000000.0),
            None => MAX_FREQUENCY,
        }
    }

    pub fn get_frequency(&self) -> f64 {
        match self.settings.get(Self::FREQUENCY_MEMBER) {
            Some(value) => value
                .as_f64()
                .unwrap_or(MAX_FREQUENCY)
                .clamp(1.0, 1000000.0),
            None => 0.0,
        }
    }

    pub fn get_max_block_size(&self) -> u32 {
        match self.settings.get(Self::MAX_BLOCK_SIZE_MEMBER) {
            Some(value) => value
                .as_u64()
                .unwrap_or(MAX_BLOCK_SIZE as u64)
                .clamp(1, 8192) as u32,
            None => MAX_BLOCK_SIZE,
        }
    }

    pub fn get_max_state_size(&self) -> u64 {
        match self.settings.get(Self::MAX_STATE_SIZE_MEMBER) {
            Some(value) => value
                .as_u64()
                .unwrap_or(MAX_STATE_SIZE)
                .clamp(8192, 1024 * 1024 * 1024 + 1),
            None => MAX_STATE_SIZE,
        }
    }

    pub fn get_max_stack_size(&self) -> u64 {
        match self.settings.get(Self::MAX_STACK_SIZE_MEMBER) {
            Some(value) => value
                .as_u64()
                .unwrap_or(MAX_STACK_SIZE)
                .clamp(1, 1024 * 1024 * 1024 + 1),
            None => MAX_STACK_SIZE,
        }
    }

    pub fn get_event_buffer_size(&self) -> u32 {
        match self.settings.get(Self::EVENT_BUFFER_SIZE_MEMBER) {
            Some(value) => value
                .as_u64()
                .unwrap_or(EVENT_BUFFER_SIZE as u64)
                .clamp(1, 8192) as u32,
            None => EVENT_BUFFER_SIZE,
        }
    }

    pub fn get_max_alloc_pool_size(&self) -> usize {
        match self.settings.get(Self::MAX_POOL_SIZE_MEMBER) {
            Some(value) => value
                .as_u64()
                .unwrap_or(MAX_POOL_SIZE as u64)
                .clamp(0, 1024 * 1024 * 1024) as usize,
            None => MAX_POOL_SIZE as usize,
        }
    }

    pub fn get_optimization_level(&self) -> i32 {
        match self.settings.get(Self::OPTIMIZATION_LEVEL_MEMBER) {
            Some(value) => value.as_i64().unwrap_or(-1).clamp(-1, 5) as i32,
            None => -1,
        }
    }

    pub fn get_session_id(&self) -> i32 {
        match self.settings.get(Self::SESSION_ID_MEMBER) {
            Some(value) => value.as_i64().unwrap_or(0) as i32,
            None => 0,
        }
    }

    pub fn should_ignore_warnings(&self) -> bool {
        match self.settings.get(Self::IGNORE_WARNING_MEMBER) {
            Some(value) => value.as_bool().unwrap_or(false),
            None => false,
        }
    }

    pub fn should_dump_debug_info(&self) -> bool {
        match self.settings.get(Self::DEBUG_MEMBER) {
            Some(value) => value.as_bool().unwrap_or(false),
            None => false,
        }
    }

    pub fn debug_flag_set(&self) -> bool {
        match self.settings.get(Self::DEBUG_MEMBER) {
            Some(value) => value.as_bool().unwrap_or(false),
            None => false,
        }
    }

    pub fn should_use_fast_maths(&self) -> bool {
        self.get_optimization_level() >= 4
    }

    pub fn get_main_processor(&self) -> String {
        match self.settings.get(Self::MAIN_PROCESSOR_MEMBER) {
            Some(value) => value.as_str().unwrap_or("").to_string(),
            None => String::from(""),
        }
    }

    pub fn set_max_frequency(&mut self, freq: f64) -> &mut Self {
        self.settings[Self::MAX_FREQUENCY_MEMBER] = freq.into();
        self
    }

    pub fn set_frequency(&mut self, freq: f64) -> &mut Self {
        self.settings[Self::FREQUENCY_MEMBER] = freq.into();
        self
    }

    pub fn set_max_block_size(&mut self, size: u32) -> &mut Self {
        self.settings[Self::MAX_BLOCK_SIZE_MEMBER] = size.into();
        self
    }

    pub fn set_max_state_size(&mut self, size: u64) -> &mut Self {
        self.settings[Self::MAX_STATE_SIZE_MEMBER] = size.into();
        self
    }

    pub fn set_max_stack_size(&mut self, size: u64) -> &mut Self {
        self.settings[Self::MAX_STACK_SIZE_MEMBER] = size.into();
        self
    }

    pub fn set_event_buffer_size(&mut self, size: u32) -> &mut Self {
        self.settings[Self::EVENT_BUFFER_SIZE_MEMBER] = size.into();
        self
    }

    pub fn set_max_pool_size(&mut self, size: usize) -> &mut Self {
        self.settings[Self::MAX_POOL_SIZE_MEMBER] = size.into();
        self
    }

    pub fn set_optimization_level(&mut self, level: u32) -> &mut Self {
        self.settings[Self::OPTIMIZATION_LEVEL_MEMBER] = level.into();
        self
    }

    pub fn set_session_id(&mut self, id: u32) -> &mut Self {
        self.settings[Self::SESSION_ID_MEMBER] = id.into();
        self
    }

    pub fn set_debug_flag(&mut self, debug: bool) -> &mut Self {
        self.settings[Self::DEBUG_MEMBER] = debug.into();
        self
    }

    pub fn set_main_processor(&mut self, s: &str) -> &mut Self {
        self.settings[Self::MAIN_PROCESSOR_MEMBER] = s.into();
        self
    }

    pub fn reset(&mut self) {
        self.settings = Value::Object(Map::new());
    }

    pub fn to_json(&self) -> String {
        self.settings.to_string()
    }

    pub fn get_value(&self) -> &Value {
        &self.settings
    }

    pub fn merge_values(&mut self, other: &BuildSettings) {
        if !self.settings.is_object() {
            self.settings = other.settings.clone();
        } else if other.settings.is_object() {
            for (name, value) in other.settings.as_object().unwrap() {
                self.settings[name] = value.clone();
            }
        }
    }

    // Private methods

    /*fn get<'a, T>(&'a self, name: &str) -> Option<T> where Value: Into<T> {
        let value = self
            .settings
            .get(name);

        if let Some(value) = value {
            Some(value.clone().into())
        } else {
            None
        }
    }

    fn get_with_default<'a, T>(&'a self, name: &str, default_value: T) -> T where T: From<&'a Value> {
        if self.settings.is_object() {
            if let Some(value) = self.settings.get(name) {
                return value.into();
            }
        }

        default_value
    }

    fn get_with_range_check<'a, T>(&'a self, name: &str, min_value: T, max_value: T, default_value: T) -> T where T: PartialOrd + From<Value> {
        let value = Value::from(0.0f64);
        let float: f64 = value.into();
        if self.settings.is_object() {
            if let Some(value) = self.settings.get(name) {
                let value = value.clone().into();

                if value < min_value {
                    return min_value;
                }

                if value > max_value {
                    return max_value;
                }

                return value;
            }
        }

        default_value
    }

    fn set<T>(&mut self, name: &str, value: T) where T: Into<Value> {
        if !self.settings.is_object() {
            self.reset();
        }

        self.settings[name] = value.into();
    }*/
}

impl From<Value> for BuildSettings {
    fn from(value: Value) -> Self {
        Self { settings: value }
    }
}

impl From<&str> for BuildSettings {
    fn from(value: &str) -> Self {
        Self {
            settings: serde_json::from_str(value).unwrap(),
        }
    }
}
