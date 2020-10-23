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

use super::Visualizer;
use crate::transport::Transport;

type Result<T> = std::result::Result<T, eyre::Error>;

#[derive(Debug, Clone)]
pub struct Percentage {
    percentage: u8,
    color: u32,
}

impl Percentage {
    pub fn new() -> Self {
        Percentage {
            percentage: 0,
            color: 0xFF0000FF,
        }
    }
}

impl Visualizer for Percentage {
    fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    fn get_id(&self) -> String {
        "percentage".to_string()
    }

    fn get_name(&self) -> String {
        "Percentage".to_string()
    }

    fn get_description(&self) -> String {
        "Illuminates a certain percentage of the keyboard".to_string()
    }

    fn render(&self, transport: &dyn Transport) -> Result<()> {
        Ok(())
    }
}
