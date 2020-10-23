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

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, eyre::Error>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub version: i32,
    pub exe_file: PathBuf,
    pub process_name: String,
    pub window_instance: String,
    pub checksum: String,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub description: String,
    pub location: usize,
    pub default_color: u32,
}

impl Manifest {
    pub fn from_file<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let s = fs::read_to_string(filename.as_ref())?;
        let result = serde_yaml::from_str(&s)?;

        Ok(result)
    }
}
