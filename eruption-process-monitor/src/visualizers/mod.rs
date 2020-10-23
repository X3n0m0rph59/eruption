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

use std::sync::Arc;

use crate::transport::Transport;
use dyn_clonable::*;
use lazy_static::lazy_static;
use log::*;
use parking_lot::Mutex;

mod percentage;
mod solid_color;

pub use percentage::*;
pub use solid_color::*;

type Result<T> = std::result::Result<T, eyre::Error>;

lazy_static! {
    pub(crate) static ref VISUALIZERS: Arc<Mutex<Vec<Box<dyn Visualizer + Send + Sync + 'static>>>> =
        Arc::new(Mutex::new(vec![]));
}

#[clonable]
pub trait Visualizer: Clone {
    fn initialize(&mut self) -> Result<()>;

    fn get_id(&self) -> String;
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;

    fn render(&self, transport: &dyn Transport) -> Result<()>;
}

/// Register a visualizer
pub fn register_visualizer<V>(visualizer: V)
where
    V: Visualizer + Send + Sync + 'static,
{
    info!(
        "{} - {}",
        visualizer.get_name(),
        visualizer.get_description()
    );

    VISUALIZERS.lock().push(Box::from(visualizer));
}

/// Register all available visualizers
pub fn register_visualizers() -> Result<()> {
    info!("Registering data visualizer plugins:");

    register_visualizer(SolidColor::new());
    register_visualizer(Percentage::new());

    // initialize all registered visualizers
    for s in VISUALIZERS.lock().iter_mut() {
        s.initialize()?;
    }

    Ok(())
}

/// Find a visualizer by its respective id
pub fn find_visualizer_by_id(id: &str) -> Option<Box<dyn Visualizer + Send + Sync + 'static>> {
    match VISUALIZERS.lock().iter().find(|&e| e.get_id() == id) {
        Some(s) => Some(dyn_clone::clone_box(s.as_ref().clone())),

        None => None,
    }
}
