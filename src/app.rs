use std::sync::mpsc::TryRecvError;

use crate::config_manager::{ConfigManager, Settings};
use crate::timer::{Timer, TimerCommand, TimerEvent, TimerHandle};

pub struct PomodoroApp {
    config: ConfigManager,
    timer_handle: Option<TimerHandle>,
}

impl PomodoroApp {
    pub fn new() -> Self {
        Self {
            config: ConfigManager::new(),
            timer_handle: None,
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
        self.timer_handle = Some(Timer::spawn(self.config.get_settings().clone()));
    } 

    pub fn pause_timer(&self) {
        if let Some(handle) = &self.timer_handle {
            let _ = handle.cmd_tx.send(TimerCommand::Pause);
        }
    }

    pub fn resume_timer(&self) {
        if let Some(handle) = &self.timer_handle {
            let _ = handle.cmd_tx.send(TimerCommand::Resume);
        }
    }

    pub fn stop_timer(&self) {
        if let Some(handle) = &self.timer_handle {
            let _ = handle.cmd_tx.send(TimerCommand::Stop);
        }
    }

    pub fn advance_timer(&self) {
        if let Some(handle) = &self.timer_handle {
            let _ = handle.cmd_tx.send(TimerCommand::Next);
        }
    }

    pub fn poll_timer_event(&mut self) -> Option<TimerEvent> {
        if let Some(handle) = &self.timer_handle {
            match handle.evt_rx.try_recv() {
                Ok(evt) => Some(evt),
                Err(TryRecvError::Empty) => None,
                Err(TryRecvError::Disconnected) => {
                    // If we are done with the timer
                    self.timer_handle = None;
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn is_timer_disconnected(&self) -> bool {
        if let None = self.timer_handle {true} else {false}
    }
}