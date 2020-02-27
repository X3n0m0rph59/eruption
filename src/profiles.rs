/*
    This file is part of Eruption.

    Eruption is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Eruption is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Eruption.  If not, see <http://www.gnu.org/licenses/>.
*/

use crate::constants;
use failure::Fail;
use log::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::default::Default;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, ProfileError>;

#[derive(Debug, Fail)]
pub enum ProfileError {
    #[fail(display = "Could not open profile file for reading")]
    OpenError {},

    #[fail(display = "Could not parse profile file")]
    ParseError {},

    #[fail(display = "Could not save profile file: {}", msg)]
    WriteError { msg: String },

    #[fail(display = "Could not find profile file from UUID")]
    FindError {},

    #[fail(display = "Could not set a config value in a profile: {}", msg)]
    SetValueError { msg: String },
    // #[fail(display = "Unknown error: {}", description)]
    // UnknownError { description: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ConfigParam {
    Int { name: String, value: i64 },
    Float { name: String, value: f64 },
    Bool { name: String, value: bool },
    String { name: String, value: String },
    Color { name: String, value: u32 },
}

pub trait GetAttr {
    fn get_name(&self) -> &String;
    fn get_value(&self) -> String;
}

impl GetAttr for ConfigParam {
    fn get_name(&self) -> &String {
        match self {
            ConfigParam::Int { ref name, .. } => name,

            ConfigParam::Float { ref name, .. } => name,

            ConfigParam::Bool { ref name, .. } => name,

            ConfigParam::String { ref name, .. } => name,

            ConfigParam::Color { ref name, .. } => name,
        }
    }

    fn get_value(&self) -> String {
        match self {
            ConfigParam::Int { ref value, .. } => format!("{}", value),

            ConfigParam::Float { ref value, .. } => format!("{}", value),

            ConfigParam::Bool { ref value, .. } => format!("{}", value),

            ConfigParam::String { ref value, .. } => value.to_owned(),

            ConfigParam::Color { ref value, .. } => format!("#{:06x}", value),
        }
    }
}

fn default_id() -> Uuid {
    Uuid::new_v4()
}

fn default_profile_file() -> PathBuf {
    "".into()
}

fn default_script_file() -> Vec<PathBuf> {
    vec![constants::DEFAULT_EFFECT_SCRIPT.into()]
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    #[serde(default = "default_id")]
    pub id: Uuid,

    #[serde(default = "default_profile_file")]
    #[serde(skip_serializing)]
    pub profile_file: PathBuf,

    pub name: String,
    pub description: String,

    #[serde(default = "default_script_file")]
    pub active_scripts: Vec<PathBuf>,

    pub config: Option<HashMap<String, Vec<ConfigParam>>>,
}

pub trait FindConfig {
    fn find_config_param(&self, param: &str) -> Option<&ConfigParam>;
    fn find_config_param_mut(&mut self, param: &str) -> Option<&mut ConfigParam>;
}

impl FindConfig for Vec<ConfigParam> {
    fn find_config_param(&self, param: &str) -> Option<&ConfigParam> {
        for p in self.iter() {
            match p {
                ConfigParam::Int { name, .. } => {
                    if name == param {
                        return Some(p);
                    }
                }

                ConfigParam::Float { name, .. } => {
                    if name == param {
                        return Some(p);
                    }
                }

                ConfigParam::Bool { name, .. } => {
                    if name == param {
                        return Some(p);
                    }
                }

                ConfigParam::String { name, .. } => {
                    if name == param {
                        return Some(p);
                    }
                }

                ConfigParam::Color { name, .. } => {
                    if name == param {
                        return Some(p);
                    }
                }
            }
        }

        None
    }

    fn find_config_param_mut(&mut self, param: &str) -> Option<&mut ConfigParam> {
        for p in self.iter_mut() {
            match p {
                ConfigParam::Int { name, .. } => {
                    if name == param {
                        return Some(p);
                    }
                }

                ConfigParam::Float { name, .. } => {
                    if name == param {
                        return Some(p);
                    }
                }

                ConfigParam::Bool { name, .. } => {
                    if name == param {
                        return Some(p);
                    }
                }

                ConfigParam::String { name, .. } => {
                    if name == param {
                        return Some(p);
                    }
                }

                ConfigParam::Color { name, .. } => {
                    if name == param {
                        return Some(p);
                    }
                }
            }
        }

        None
    }
}

impl Profile {
    pub fn new(profile_file: &Path) -> Result<Self> {
        // parse manifest
        match fs::read_to_string(profile_file) {
            Ok(toml) => {
                // parse profile
                match toml::de::from_str::<Self>(&toml) {
                    Ok(mut result) => {
                        // fill in required fields, after parsing
                        result.id = Uuid::new_v4();
                        result.profile_file = profile_file.to_path_buf();

                        result.config = Some(HashMap::new());

                        Ok(result)
                    }

                    Err(_e) => Err(ProfileError::ParseError {}),
                }
            }

            Err(_e) => Err(ProfileError::OpenError {}),
        }
    }

    pub fn from(profile_file: &Path) -> Result<Self> {
        // parse manifest
        match fs::read_to_string(profile_file) {
            Ok(toml) => {
                // parse profile
                match toml::de::from_str::<Self>(&toml) {
                    Ok(mut result) => {
                        // fill in required fields, after parsing
                        result.profile_file = profile_file.to_path_buf();

                        if result.config.is_none() {
                            result.config = Some(HashMap::new());
                        }

                        Ok(result)
                    }

                    Err(_e) => Err(ProfileError::ParseError {}),
                }
            }

            Err(_e) => Err(ProfileError::OpenError {}),
        }
    }

    pub fn find_by_uuid(uuid: Uuid, profile_path: &Path) -> Result<Self> {
        let profile_files = get_profile_files(&profile_path).unwrap();
        let mut result = Err(ProfileError::FindError {});

        'PROFILE_LOOP: for profile_file in profile_files.iter() {
            match Profile::from(&profile_file) {
                Ok(profile) => {
                    if profile.id == uuid {
                        result = Ok(profile);
                        break 'PROFILE_LOOP;
                    }
                }

                Err(e) => {
                    error!(
                        "Could not process profile '{}': {}",
                        profile_file.display(),
                        e
                    );
                }
            }
        }

        result
    }

    pub fn save(&self) -> Result<()> {
        let toml = toml::ser::to_string_pretty(&self).map_err(|_| ProfileError::WriteError {
            msg: "Could not convert profile data".into(),
        })?;

        fs::write(&self.profile_file, &toml).map_err(|_| ProfileError::WriteError {
            msg: "Could not write file".into(),
        })?;

        Ok(())
    }

    pub fn get_int_value(&self, script_name: &str, name: &str) -> Option<&i64> {
        if let Some(config) = &self.config {
            if let Some(cfg) = config.get(script_name) {
                match cfg.find_config_param(name) {
                    Some(param) => match param {
                        ConfigParam::Int { value, .. } => Some(value),

                        _ => None,
                    },

                    None => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn set_int_value(&mut self, script_name: &str, name: &str, val: i64) -> Result<()> {
        if let Some(ref mut config) = self.config {
            if let Some(ref mut cfg) = config.get_mut(script_name) {
                match cfg.find_config_param_mut(name) {
                    Some(ref mut param) => match param {
                        ConfigParam::Int { ref mut value, .. } => {
                            *value = val;
                            Ok(())
                        }

                        _ => Err(ProfileError::SetValueError {
                            msg: "Invalid data type".into(),
                        }),
                    },

                    _ => {
                        cfg.push(ConfigParam::Int {
                            name: name.to_string(),
                            value: val,
                        });
                        Ok(())
                    }
                }
            } else {
                config.insert(
                    script_name.into(),
                    vec![ConfigParam::Int {
                        name: name.to_string(),
                        value: val,
                    }],
                );

                Ok(())
            }
        } else {
            Err(ProfileError::SetValueError {
                msg: "Could not get config".into(),
            })
        }
    }

    pub fn get_float_value(&self, script_name: &str, name: &str) -> Option<&f64> {
        if let Some(config) = &self.config {
            if let Some(cfg) = config.get(script_name) {
                match cfg.find_config_param(name) {
                    Some(param) => match param {
                        ConfigParam::Float { value, .. } => Some(value),
                        _ => None,
                    },
                    None => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn set_float_value(&mut self, script_name: &str, name: &str, val: f64) -> Result<()> {
        if let Some(ref mut config) = self.config {
            if let Some(ref mut cfg) = config.get_mut(script_name) {
                match cfg.find_config_param_mut(name) {
                    Some(ref mut param) => match param {
                        ConfigParam::Float { ref mut value, .. } => {
                            *value = val;
                            Ok(())
                        }

                        _ => Err(ProfileError::SetValueError {
                            msg: "Invalid data type".into(),
                        }),
                    },

                    _ => {
                        cfg.push(ConfigParam::Float {
                            name: name.to_string(),
                            value: val,
                        });
                        Ok(())
                    }
                }
            } else {
                config.insert(
                    script_name.into(),
                    vec![ConfigParam::Float {
                        name: name.to_string(),
                        value: val,
                    }],
                );

                Ok(())
            }
        } else {
            Err(ProfileError::SetValueError {
                msg: "Could not get config".into(),
            })
        }
    }

    pub fn get_bool_value(&self, script_name: &str, name: &str) -> Option<&bool> {
        if let Some(config) = &self.config {
            if let Some(cfg) = config.get(script_name) {
                match cfg.find_config_param(name) {
                    Some(param) => match param {
                        ConfigParam::Bool { value, .. } => Some(value),

                        _ => None,
                    },

                    None => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn set_bool_value(&mut self, script_name: &str, name: &str, val: bool) -> Result<()> {
        if let Some(ref mut config) = self.config {
            if let Some(ref mut cfg) = config.get_mut(script_name) {
                match cfg.find_config_param_mut(name) {
                    Some(ref mut param) => match param {
                        ConfigParam::Bool { ref mut value, .. } => {
                            *value = val;
                            Ok(())
                        }

                        _ => Err(ProfileError::SetValueError {
                            msg: "Invalid data type".into(),
                        }),
                    },

                    _ => {
                        cfg.push(ConfigParam::Bool {
                            name: name.to_string(),
                            value: val,
                        });
                        Ok(())
                    }
                }
            } else {
                config.insert(
                    script_name.into(),
                    vec![ConfigParam::Bool {
                        name: name.to_string(),
                        value: val,
                    }],
                );

                Ok(())
            }
        } else {
            Err(ProfileError::SetValueError {
                msg: "Could not get config".into(),
            })
        }
    }

    pub fn get_str_value(&self, script_name: &str, name: &str) -> Option<&str> {
        if let Some(config) = &self.config {
            if let Some(cfg) = config.get(script_name) {
                match cfg.find_config_param(name) {
                    Some(param) => match param {
                        ConfigParam::String { value, .. } => Some(value),

                        _ => None,
                    },

                    None => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn set_str_value(&mut self, script_name: &str, name: &str, val: String) -> Result<()> {
        if let Some(ref mut config) = self.config {
            if let Some(ref mut cfg) = config.get_mut(script_name) {
                match cfg.find_config_param_mut(name) {
                    Some(ref mut param) => match param {
                        ConfigParam::String { ref mut value, .. } => {
                            *value = val;
                            Ok(())
                        }

                        _ => Err(ProfileError::SetValueError {
                            msg: "Invalid data type".into(),
                        }),
                    },

                    _ => {
                        cfg.push(ConfigParam::String {
                            name: name.to_string(),
                            value: val,
                        });
                        Ok(())
                    }
                }
            } else {
                config.insert(
                    script_name.into(),
                    vec![ConfigParam::String {
                        name: name.to_string(),
                        value: val,
                    }],
                );

                Ok(())
            }
        } else {
            Err(ProfileError::SetValueError {
                msg: "Could not get config".into(),
            })
        }
    }

    pub fn get_color_value(&self, script_name: &str, name: &str) -> Option<&u32> {
        if let Some(config) = &self.config {
            if let Some(cfg) = config.get(script_name) {
                match cfg.find_config_param(name) {
                    Some(param) => match param {
                        ConfigParam::Color { value, .. } => Some(value),

                        _ => None,
                    },

                    None => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn set_color_value(&mut self, script_name: &str, name: &str, val: u32) -> Result<()> {
        if let Some(ref mut config) = self.config {
            if let Some(ref mut cfg) = config.get_mut(script_name) {
                match cfg.find_config_param_mut(name) {
                    Some(ref mut param) => match param {
                        ConfigParam::Color { ref mut value, .. } => {
                            *value = val;
                            Ok(())
                        }

                        _ => Err(ProfileError::SetValueError {
                            msg: "Invalid data type".into(),
                        }),
                    },

                    _ => {
                        cfg.push(ConfigParam::Color {
                            name: name.to_string(),
                            value: val,
                        });
                        Ok(())
                    }
                }
            } else {
                config.insert(
                    script_name.into(),
                    vec![ConfigParam::Color {
                        name: name.to_string(),
                        value: val,
                    }],
                );

                Ok(())
            }
        } else {
            Err(ProfileError::SetValueError {
                msg: "Could not get config".into(),
            })
        }
    }
}

impl Default for Profile {
    fn default() -> Self {
        let profile_file =
            Path::new(constants::DEFAULT_PROFILE_DIR).join(Path::new("default.profile"));

        let config = Some(HashMap::new());

        Self {
            id: default_id(),
            profile_file,
            name: "Default".into(),
            description: "Auto-generated profile".into(),
            active_scripts: vec![PathBuf::from(constants::DEFAULT_EFFECT_SCRIPT)],
            config,
        }
    }
}

pub fn get_profiles(profile_path: &Path) -> Result<Vec<Profile>> {
    let profile_files = get_profile_files(&profile_path).unwrap();

    let mut errors_present = false;
    let mut result: Vec<Profile> = vec![];

    for profile_file in profile_files.iter() {
        match Profile::from(&profile_file) {
            Ok(profile) => {
                result.push(profile);
            }

            Err(e) => {
                errors_present = true;
                error!(
                    "Could not process profile '{}': {}",
                    profile_file.display(),
                    e
                );
            }
        }
    }

    if errors_present {
        warn!("An error occurred during processing of profiles");
    }

    Ok(result)
}

pub fn get_profile_files(profile_path: &Path) -> Result<Vec<PathBuf>> {
    let paths = fs::read_dir(&profile_path).unwrap();

    Ok(paths
        .map(|p| p.unwrap().path())
        .filter(|p| {
            if p.extension().is_some() {
                return p.extension().unwrap() == "profile";
            }

            false
        })
        .collect())
}

#[allow(dead_code)]
pub fn find_path_by_uuid(uuid: Uuid, profile_path: &Path) -> Option<PathBuf> {
    let profile_files = get_profile_files(&profile_path).unwrap();

    let mut errors_present = false;
    let mut result = None;

    'PROFILE_LOOP: for profile_file in profile_files.iter() {
        match Profile::from(&profile_file) {
            Ok(profile) => {
                if profile.id == uuid {
                    result = Some(profile_file.to_path_buf());
                    break 'PROFILE_LOOP;
                }
            }

            Err(e) => {
                errors_present = true;
                error!(
                    "Could not process profile '{}': {}",
                    profile_file.display(),
                    e
                );
            }
        }
    }

    if errors_present {
        warn!("An error occurred during processing of profiles");
    }

    result
}
