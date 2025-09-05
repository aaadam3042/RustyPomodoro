use crate::config_manager::{ConfigManager, Settings};
use crate::timer::Timer;

pub struct PomodoroApp {
    config: ConfigManager,
    timer: Timer,
}

impl PomodoroApp {
    pub fn new() -> Self {
        Self {
            config: ConfigManager::new(),
            timer: Timer::new(),
        }
    }

    pub fn init(&mut self) {
        self.config.build();
    }

    pub fn get_settings(&self) -> &Settings {
        self.config.get_settings()
    }

    pub fn save_config() {
        
    }

    pub fn start(&self) {
        self.timer.start();
    } 
}