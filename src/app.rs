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

    pub fn save_config(&mut self, new_settings: Settings) {
        *self.config.get_mut_settings() = new_settings;
        self.config.save();
    }

    pub fn start_timer(&mut self) {
        self.timer.init(self.config.get_settings().clone());
        self.timer.start();
    } 
}