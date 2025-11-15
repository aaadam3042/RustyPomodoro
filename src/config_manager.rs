use directories::ProjectDirs;
use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fmt;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, Write};
use std::path::PathBuf;

#[derive(Default, Serialize, Deserialize)]
pub struct Settings {
    work_minutes: u32,
    relief_seconds: u32,
    break_minutes: u32,
    work_relief_cycles: u32,
}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Work for {} minutes, Rest for {} seconds, for {} cycles, then break for {} minutes", 
            self.work_minutes, self.relief_seconds, self.work_relief_cycles, self.break_minutes
        )
    }
}

pub struct ConfigManager{
    settings: Settings,
}

impl ConfigManager{
    pub fn new() -> Self {
        Self { 
            settings: Settings::default()
        }
    }

    pub fn build(&mut self){
        let result = ConfigManager::get_settings_from_file();
        match result {
            Ok(v) => {
                // Modify current settings
                self.settings = v;
            }
            Err(_) => {
                // Create default settings. Also attempt to save the settings to a json file. If save fails nothing changes.
                self.settings = Settings {work_minutes: 20, relief_seconds: 20, break_minutes: 5, work_relief_cycles: 2};
                self.save_settings_to_json();
            }
        }
    }

    pub fn get_settings(&self) -> &Settings {
        &self.settings
    }

    fn get_config_file_path() -> Result<PathBuf, Box<dyn Error>> {
        let proj_dirs = ProjectDirs::from("com", "aaadam3042", "rustypomodoro")
            .ok_or("Could not determine correct config directory")?;
        let config_dir = proj_dirs.config_dir();
        create_dir_all(config_dir)?;
        Ok(config_dir.join("config.json"))
    }

    fn get_settings_from_file() -> Result<Settings, Box<dyn Error>> {
        let path = Self::get_config_file_path()?;
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let data: Settings = serde_json::from_reader(reader)?;
        Ok(data)
    }

    fn save_settings_to_json(&self) {
        // Perform serialization and save here - failures will return early and not interrupt app function
        let json_string = match serde_json::to_string_pretty(&self.settings){
            Ok(s) => s,
            Err(e) => {
                eprintln!("Warning: could not serialize settings: {}", e);
                return;
            }
        };
    
        let path = match Self::get_config_file_path() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Warning: Could not determine correct config directory: {}", e);
                return;
            }
        };

        let mut file = match File::create(&path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Warning: Could not create appropriate config file: {}", e);
                return;
            }
        };

        if let Err(e) = file.write_all(json_string.as_bytes()) {
            eprintln!("Warning: could not write settings to file: {}", e);
        }
    }

}
