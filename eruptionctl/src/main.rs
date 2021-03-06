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

use clap::Clap;
use colored::*;
use dbus::nonblock;
use dbus::nonblock::stdintf::org_freedesktop_dbus::Properties;
use dbus_tokio::connection;
use manifest::GetAttr;
use profiles::GetAttr as GetAttrProfile;
use std::sync::Arc;
use std::time::Duration;
use std::{collections::HashMap, path::PathBuf};

mod constants;
mod dbus_client;
mod manifest;
mod profiles;
mod util;

type Result<T> = std::result::Result<T, eyre::Error>;

#[derive(Debug, thiserror::Error)]
pub enum MainError {
    #[error("Unknown error: {description}")]
    UnknownError { description: String },
}

/// Supported command line arguments
#[derive(Debug, Clap)]
#[clap(
    version = env!("CARGO_PKG_VERSION"),
    author = "X3n0m0rph59 <x3n0m0rph59@gmail.com>",
    about = "A CLI control utility for the Eruption Linux user-mode driver",
)]
pub struct Options {
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,

    #[clap(subcommand)]
    command: Subcommands,
}

// Sub-commands
#[derive(Debug, Clap)]
pub enum Subcommands {
    /// Configuration related sub-commands
    Config {
        #[clap(subcommand)]
        command: ConfigSubcommands,
    },

    /// Shows the currently active profile or slot
    Status {
        #[clap(subcommand)]
        command: StatusSubcommands,
    },

    /// Switch to a different profile or slot
    Switch {
        #[clap(subcommand)]
        command: SwitchSubcommands,
    },

    /// Profile related sub-commands
    Profiles {
        #[clap(subcommand)]
        command: ProfilesSubcommands,
    },

    /// Naming related commands such as renaming of profile slots
    Names {
        #[clap(subcommand)]
        command: NamesSubcommands,
    },

    /// Script related subcommands
    Scripts {
        #[clap(subcommand)]
        command: ScriptsSubcommands,
    },

    /// Get or set script parameters on the currently active profile
    Param {
        script: Option<String>,
        parameter: Option<String>,
        value: Option<String>,
    },

    /// Generate shell completions
    Completions {
        #[clap(subcommand)]
        command: CompletionsSubcommands,
    },
}

/// Sub-commands of the "config" command
#[derive(Debug, Clap)]
pub enum ConfigSubcommands {
    /// Get or set the brightness of the LEDs
    Brightness { brightness: Option<i64> },

    /// Get or set the state of SoundFX
    Soundfx { enable: Option<bool> },
}

/// Sub-commands of the "status" command
#[derive(Debug, Clap)]
pub enum StatusSubcommands {
    /// Shows the currently active profile
    Profile,

    /// Shows the currently active slot
    Slot,
}

/// Sub-commands of the "switch" command
#[derive(Debug, Clap)]
pub enum SwitchSubcommands {
    /// Switch profiles
    Profile { profile_name: String },

    /// Switch slots
    Slot { index: usize },
}

/// Sub-commands of the "profiles" command
#[derive(Debug, Clap)]
pub enum ProfilesSubcommands {
    /// Show info about a profile
    Info { profile_name: String },

    /// Edit a profile
    Edit { profile_name: String },

    /// List available profiles
    List,
}

/// Subcommands of the "names" command
#[derive(Debug, Clap)]
pub enum NamesSubcommands {
    /// List slot names
    List,

    /// Set the name of a single profile slot
    Set { slot_index: usize, name: String },

    /// Set all the profile slot names at once
    SetAll { names: Vec<String> },
}

/// Subcommands of the "scripts" command
#[derive(Debug, Clap)]
pub enum ScriptsSubcommands {
    /// Show info about a script
    Info { script_name: String },

    /// Edit a script
    Edit { script_name: String },

    /// List available scripts
    List,
}

/// Subcommands of the "completions" command
#[derive(Debug, Clap)]
pub enum CompletionsSubcommands {
    Bash,

    Elvish,

    Fish,

    PowerShell,

    Zsh,
}

/// Print license information
#[allow(dead_code)]
fn print_header() {
    println!(
        r#"
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
"#
    );
}

/// Returns a connection to the D-Bus system bus using the specified `path`
pub async fn dbus_system_bus(
    path: &str,
) -> Result<dbus::nonblock::Proxy<'_, Arc<dbus::nonblock::SyncConnection>>> {
    let (resource, conn) = connection::new_system_sync()?;

    tokio::spawn(async {
        let err = resource.await;
        panic!("Lost connection to D-Bus: {}", err);
    });

    let proxy = nonblock::Proxy::new(
        "org.eruption",
        path,
        Duration::from_secs(constants::DBUS_TIMEOUT_MILLIS as u64),
        conn,
    );

    Ok(proxy)
}

/// Switch the currently active profile
pub async fn switch_profile(name: &str) -> Result<()> {
    let file_name = name.to_owned();

    let (_result,): (bool,) = dbus_system_bus("/org/eruption/profile")
        .await?
        .method_call("org.eruption.Profile", "SwitchProfile", (file_name,))
        .await?;

    Ok(())
}

/// Get the index of the currently active slot
pub async fn get_active_slot() -> Result<usize> {
    let result: u64 = dbus_system_bus("/org/eruption/slot")
        .await?
        .get("org.eruption.Slot", "ActiveSlot")
        .await?;

    Ok(result as usize)
}

/// Get the name of the currently active profile
pub async fn get_active_profile() -> Result<String> {
    let result: String = dbus_system_bus("/org/eruption/profile")
        .await?
        .get("org.eruption.Profile", "ActiveProfile")
        .await?;

    Ok(result)
}

/// Switch the currently active slot
pub async fn switch_slot(index: usize) -> Result<()> {
    let (_result,): (bool,) = dbus_system_bus("/org/eruption/slot")
        .await?
        .method_call("org.eruption.Slot", "SwitchSlot", (index as u64,))
        .await?;

    Ok(())
}

/// Get the names of the profile slots
pub async fn get_slot_names() -> Result<Vec<String>> {
    let result: Vec<String> = dbus_system_bus("/org/eruption/slot")
        .await?
        .get("org.eruption.Slot", "SlotNames")
        .await?;

    Ok(result)
}

/// Set the names of the profile slots
pub async fn set_slot_names(names: &[String]) -> Result<()> {
    let arg = Box::new(names);

    let _result = dbus_system_bus("/org/eruption/slot")
        .await?
        .set("org.eruption.Slot", "SlotNames", arg)
        .await?;

    Ok(())
}

/// Set the name of a single profile slot
pub async fn set_slot_name(slot_index: usize, name: String) -> Result<()> {
    let mut result = get_slot_names().await?;

    result[slot_index] = name;
    set_slot_names(&result).await?;

    Ok(())
}

/// Enumerate all available profiles
pub async fn get_profiles() -> Result<Vec<(String, String)>> {
    let (result,): (Vec<(String, String)>,) = dbus_system_bus("/org/eruption/profile")
        .await?
        .method_call("org.eruption.Profile", "EnumProfiles", ())
        .await?;

    Ok(result)
}

/// Enumerate all available scripts
pub fn get_script_list() -> Result<Vec<(String, String)>> {
    let path = constants::DEFAULT_SCRIPT_DIR;
    let scripts = util::enumerate_scripts(&path)?;

    let result = scripts
        .iter()
        .map(|s| {
            (
                format!("{} - {}", s.name.clone(), s.description.clone()),
                s.script_file.to_string_lossy().to_string(),
            )
        })
        .collect();

    Ok(result)
}

// global configuration options

/// Get the current brightness value
pub async fn get_brightness() -> Result<i64> {
    let result = dbus_system_bus("/org/eruption/config")
        .await?
        .get("org.eruption.Config", "Brightness")
        .await?;

    Ok(result)
}

/// Set the current brightness value
pub async fn set_brightness(brightness: i64) -> Result<()> {
    let arg = Box::new(brightness as i64);

    let _result = dbus_system_bus("/org/eruption/config")
        .await?
        .set("org.eruption.Config", "Brightness", arg)
        .await?;

    Ok(())
}

/// Returns true when SoundFX is enabled
pub async fn get_sound_fx() -> Result<bool> {
    let result = dbus_system_bus("/org/eruption/config")
        .await?
        .get("org.eruption.Config", "EnableSfx")
        .await?;

    Ok(result)
}

/// Set SoundFX state to `enabled`
pub async fn set_sound_fx(enabled: bool) -> Result<()> {
    let arg = Box::new(enabled);

    let _result = dbus_system_bus("/org/eruption/config")
        .await?
        .set("org.eruption.Config", "EnableSfx", arg)
        .await?;

    Ok(())
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
pub async fn main() -> std::result::Result<(), eyre::Error> {
    color_eyre::install()?;

    // if unsafe { libc::isatty(0) != 0 } {
    //     print_header();
    // }

    let opts = Options::parse();
    match opts.command {
        // configuration related sub-commands
        Subcommands::Config { command } => match command {
            ConfigSubcommands::Brightness { brightness } => {
                if let Some(brightness) = brightness {
                    set_brightness(brightness).await?
                } else {
                    let result = get_brightness().await?;
                    println!(
                        "{}",
                        format!("Brightness: {}", format!("{}%", result).bold())
                    );
                }
            }

            ConfigSubcommands::Soundfx { enable } => {
                if let Some(enable) = enable {
                    set_sound_fx(enable).await?
                } else {
                    let result = get_sound_fx().await?;
                    println!(
                        "{}",
                        format!("SoundFX enabled: {}", format!("{}", result).bold())
                    );
                }
            }
        },

        // profile related sub-commands
        Subcommands::Profiles { command } => match command {
            ProfilesSubcommands::Edit { profile_name } => {
                let path = constants::DEFAULT_PROFILE_DIR;
                let profiles = util::enumerate_profiles(&path)?;

                if let Some(profile) = profiles.iter().find(|p| {
                    *p.profile_file
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        == profile_name
                }) {
                    util::edit_file(&profile.profile_file)?
                } else {
                    eprintln!("No matches found");
                }
            }

            ProfilesSubcommands::List => {
                for p in get_profiles().await? {
                    println!("{}: {}", p.0.bold(), p.1);
                }
            }

            ProfilesSubcommands::Info { profile_name } => {
                let path = constants::DEFAULT_PROFILE_DIR;
                let profiles = util::enumerate_profiles(path)?;

                let empty = HashMap::new();

                if let Some(profile) = profiles.iter().find(|p| {
                    *p.profile_file
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        == profile_name
                }) {
                    println!(
                        "Profile:\t{} ({})\nDescription:\t{}\nScripts:\t{:?}\n\n{:#?}",
                        profile.name,
                        profile.id,
                        profile.description,
                        profile.active_scripts,
                        profile.config.as_ref().unwrap_or(&empty),
                    );
                } else {
                    eprintln!("No matches found");
                }
            }
        },

        // naming related sub-commands
        Subcommands::Names { command } => match command {
            NamesSubcommands::List => {
                let slot_names = get_slot_names().await?;

                for (index, name) in slot_names.iter().enumerate() {
                    let s = format!("{}", index + 1);
                    println!("{}: {}", s.bold(), name);
                }
            }

            NamesSubcommands::Set { slot_index, name } => {
                if slot_index > 0 && slot_index <= constants::NUM_SLOTS {
                    set_slot_name(slot_index - 1, name).await?;
                } else {
                    eprintln!("Slot index out of bounds");
                }
            }

            NamesSubcommands::SetAll { names } => {
                if names.len() == constants::NUM_SLOTS {
                    set_slot_names(&names).await?;
                } else {
                    eprintln!("Elements do not match number of slots");
                }
            }
        },

        // script related sub-commands
        Subcommands::Scripts { command } => match command {
            ScriptsSubcommands::Edit { script_name } => {
                let path = constants::DEFAULT_SCRIPT_DIR;
                let scripts = util::enumerate_scripts(&path)?;

                if let Some(script) = scripts.iter().find(|s| {
                    *s.script_file
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        == script_name
                }) {
                    util::edit_file(&script.script_file)?
                } else {
                    eprintln!("No matches found");
                }
            }

            ScriptsSubcommands::List => {
                for s in get_script_list()? {
                    println!("{}: {}", s.0.bold(), s.1);
                }
            }

            ScriptsSubcommands::Info { script_name } => {
                let path = constants::DEFAULT_SCRIPT_DIR;
                let scripts = util::enumerate_scripts(&path)?;

                let empty = vec![];

                if let Some(script) = scripts.iter().find(|s| {
                    *s.script_file
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        == script_name
                }) {
                    println!(
                        "Lua script:\t{} ({})\nDaemon version:\t{}\nAuthor:\t\t{}\nDescription:\t{}\nTags:\t\t{:?}",
                        script.name,
                        script.version,
                        script.min_supported_version,
                        script.author,
                        script.description,
                        script.tags.as_ref().unwrap_or(&empty),
                    );
                } else {
                    eprintln!("No matches found");
                }
            }
        },

        // parameter
        Subcommands::Param {
            script,
            parameter,
            value,
        } => {
            // determine mode of operation
            if script.is_none() && parameter.is_none() && value.is_none() {
                // list parameters from all scripts in the currently active profile
                let profile_name = get_active_profile().await.map_err(|e| {
                    eprintln!("Could not determine the currently active profile! Is the Eruption daemon running?");
                    e
                })?;

                // TODO: Get profile dir from the daemon
                let path = constants::DEFAULT_PROFILE_DIR;
                let profiles = util::enumerate_profiles(path)?;

                if let Some(profile) = profiles.iter().find(|&p| {
                    *p.profile_file
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        == profile_name
                }) {
                    println!(
                        "Profile:\t{} ({})\nDescription:\t{}\nScripts:\t{:?}\n",
                        profile.name, profile.id, profile.description, profile.active_scripts,
                    );

                    // dump parameters set in .profile file

                    println!("Profile parameters:\n");

                    let empty = HashMap::new();

                    let path = constants::DEFAULT_SCRIPT_DIR;
                    let scripts = util::enumerate_scripts(&path)?;

                    for script in &scripts {
                        if profile.active_scripts.contains(&PathBuf::from(
                            script.script_file.file_name().unwrap_or_default(),
                        )) {
                            let config = profile.config.as_ref().unwrap_or_else(|| &empty);
                            let config_params = config.get(&script.name);

                            if let Some(config_params) = config_params {
                                for config in config_params.iter() {
                                    // read param value
                                    let value = if config.get_value() == config.get_default() {
                                        format!("{}", &config.get_value()).normal()
                                    } else {
                                        format!("{}", &config.get_value()).bold()
                                    };

                                    println!(
                                        "\"{}\" {} {} (default: {})",
                                        &script.name,
                                        &config.get_name(),
                                        &value,
                                        &config.get_default(),
                                    );
                                }
                            }
                        }
                    }

                    if opts.verbose > 0 {
                        // dump all available parameters that could be set in the .profile file
                        println!();
                        println!("Available parameters:\n");

                        for script in &scripts {
                            if profile.active_scripts.contains(&PathBuf::from(
                                script.script_file.file_name().unwrap_or_default(),
                            )) {
                                if let Some(config_params) = script.config.as_ref() {
                                    for config in config_params.iter() {
                                        // read param defaults
                                        let value = config.get_default();

                                        println!(
                                            "\"{}\" {} (default: {})",
                                            &script.name,
                                            &config.get_name(),
                                            &value,
                                        );
                                    }
                                }

                                println!();
                            }
                        }
                    }
                } else {
                    eprintln!("Could not load the current profile");
                }
            } else if let Some(script) = script {
                let profile_name = get_active_profile().await.map_err(|e| {
                    eprintln!("Could not determine the currently active profile! Is the Eruption daemon running?");
                    e
                })?;

                if let Some(value) = value {
                    // set a parameter from the specified script in the currently active profile

                    let parameter = parameter.unwrap();

                    let path = constants::DEFAULT_PROFILE_DIR;
                    let profiles = util::enumerate_profiles(path)?;

                    if let Some(profile) = profiles.iter().find(|&p| {
                        *p.profile_file
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            == profile_name
                    }) {
                        println!(
                            "Profile:\t{} ({})\nDescription:\t{}\nScripts:\t{:?}\n",
                            profile.name, profile.id, profile.description, profile.active_scripts,
                        );

                        let path = constants::DEFAULT_SCRIPT_DIR;
                        let scripts = util::enumerate_scripts(&path)?;

                        for scr in scripts {
                            if scr.name == script {
                                // set param value
                                dbus_client::set_parameter(
                                    &profile.profile_file.to_string_lossy(),
                                    &scr.script_file.to_string_lossy(),
                                    &parameter,
                                    &value,
                                )?;

                                println!("\"{}\" {} {}", &scr.name, &parameter, &value.bold(),);

                                break;
                            }
                        }
                    } else {
                        eprintln!("Could not load the current profile");
                    }
                } else if let Some(parameter) = parameter {
                    // list parameters from the specified script in the currently active profile

                    let path = constants::DEFAULT_PROFILE_DIR;
                    let profiles = util::enumerate_profiles(path)?;

                    if let Some(profile) = profiles.iter().find(|&p| {
                        *p.profile_file
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            == profile_name
                    }) {
                        let empty = HashMap::new();

                        let path = constants::DEFAULT_SCRIPT_DIR;
                        let scripts = util::enumerate_scripts(&path)?;

                        'OUTER_LOOP: for script in scripts {
                            if profile.active_scripts.contains(&PathBuf::from(
                                script.script_file.file_name().unwrap_or_default(),
                            )) {
                                let config = profile.config.as_ref().unwrap_or_else(|| &empty);
                                if let Some(config) = config.get(&script.name) {
                                    for config in config.iter() {
                                        if config.get_name() == &parameter {
                                            if let Some(value) = &value {
                                                println!(
                                                    "Profile:\t{} ({})\nDescription:\t{}\nScripts:\t{:?}\n",
                                                    profile.name, profile.id, profile.description, profile.active_scripts,
                                                );

                                                // set param value
                                                dbus_client::set_parameter(
                                                    &profile.profile_file.to_string_lossy(),
                                                    &script.script_file.to_string_lossy(),
                                                    &parameter,
                                                    &value,
                                                )?;

                                                println!(
                                                    "\"{}\" {} {}",
                                                    &script.name,
                                                    &parameter,
                                                    &value.bold(),
                                                );

                                                break 'OUTER_LOOP;
                                            } else {
                                                println!(
                                                    "Profile:\t{} ({})\nDescription:\t{}\nScripts:\t{:?}\n",
                                                    profile.name, profile.id, profile.description, profile.active_scripts,
                                                );

                                                // read param value
                                                println!(
                                                    "\"{}\" {} {}",
                                                    &script.name,
                                                    &config.get_name(),
                                                    &config.get_value().bold(),
                                                );

                                                break 'OUTER_LOOP;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        eprintln!("No matches found");
                    }
                } else {
                    // list parameters from the specified script
                    println!("Dumping all parameters from the specified script:\n");

                    let path = constants::DEFAULT_SCRIPT_DIR;
                    let scripts = util::enumerate_scripts(&path)?;

                    for scr in scripts {
                        if scr.name == script {
                            for param in scr.config.unwrap_or_else(|| Vec::new()) {
                                println!(
                                    "\"{}\" {} default: {}",
                                    scr.name,
                                    param.get_name().bold(),
                                    param.get_default()
                                );
                            }
                        }
                    }
                }
            } else {
                println!("Nothing to do");
            }
        }

        // convenience operations: switch profile or slot
        Subcommands::Status { command } => match command {
            StatusSubcommands::Profile => {
                let profile_name = get_active_profile().await?;
                println!("Current profile: {}", profile_name.bold());
            }

            StatusSubcommands::Slot => {
                let index = get_active_slot().await? + 1;
                println!("Current slot: {}", format!("{}", index).bold());
            }
        },

        // convenience operations: switch profile or slot
        Subcommands::Switch { command } => match command {
            SwitchSubcommands::Profile { profile_name } => {
                println!("Switching to profile: {}", profile_name.bold());
                switch_profile(&profile_name).await?
            }

            SwitchSubcommands::Slot { index } => {
                println!("Switching to slot: {}", format!("{}", index).bold());
                let index = index - 1;
                switch_slot(index).await?
            }
        },

        Subcommands::Completions { command } => {
            use clap::IntoApp;
            use clap_generate::{generate, generators::*};

            const BIN_NAME: &str = env!("CARGO_PKG_NAME");

            let mut app = Options::into_app();
            let mut fd = std::io::stdout();

            match command {
                CompletionsSubcommands::Bash => {
                    generate::<Bash, _>(&mut app, BIN_NAME, &mut fd);
                }

                CompletionsSubcommands::Elvish => {
                    generate::<Elvish, _>(&mut app, BIN_NAME, &mut fd);
                }

                CompletionsSubcommands::Fish => {
                    generate::<Fish, _>(&mut app, BIN_NAME, &mut fd);
                }

                CompletionsSubcommands::PowerShell => {
                    generate::<PowerShell, _>(&mut app, BIN_NAME, &mut fd);
                }

                CompletionsSubcommands::Zsh => {
                    generate::<Zsh, _>(&mut app, BIN_NAME, &mut fd);
                }
            }
        }
    };

    Ok(())
}
