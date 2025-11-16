use std::error::Error;

use crate::config_manager::Settings;

// Enum to keep timer states
enum TimerState {
    Idle,
    CountDown,
    Waiting,
    Paused,
}

enum TimerSession {
    Working,
    Resting,
    Break
}

pub struct Timer {
    is_ready: bool,
    timer_settings: Settings,
    current_state: TimerState,
    current_session: TimerSession,
    time_remaining: u32,
    cycles_complete: u32,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            is_ready: false,
            timer_settings: Settings::default(),
            current_state: TimerState::Idle,
            current_session: TimerSession::Break,
            time_remaining: 0,
            cycles_complete: 0,
        }
    }

    pub fn init(&mut self, settings: Settings) {
        // Init everything other than session/start specific variables
        self.timer_settings = settings;
        self.is_ready = true;
    }

    pub fn deinit(&mut self) {
        // Ensure we deinit when we return to main menu so that, init and reloading 
        // of settings is performed before starting the timer
        self.is_ready = false;
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        if !self.is_ready {
            return Err("Tried to start timer without initialising.".into());
        }

        // This is the expected state when a timer first 'starts'
        self.current_state = TimerState::CountDown;
        self.current_session = TimerSession::Working;
        self.time_remaining = self.timer_settings.work_seconds;

        // Start a timer thread
        self.run();
        
        Ok(())
    }

    fn run(&mut self) {
        loop {  // TODO: time_remaining-- (are we idleing to wait for a second during every loop)
            match self.current_state {
                // We shouldn't technically be in idle, as timer is only here when it isnt running
                TimerState::Idle => panic!("Program somehow is in idle state while running"),
                TimerState::CountDown => {
                    if self.time_remaining <= 0 {
                        // If timer finished this session
                        self.current_state = TimerState::Waiting;
                    }
                    // TODO: Pause - non blocking input
                },
                TimerState::Waiting => {
                    // TODO: Play Audio on interval until input - non blocking?

                    // TODO: Await user confimation for next or return to menu - blocking input?
                    // TODO: If next - there should also be if exit
                    match self.current_session {
                        TimerSession::Working => {
                            self.cycles_complete += 1;
                            if self.cycles_complete == self.timer_settings.work_relief_cycles {
                                // Go straight to break because there is no point resting then breaking.
                                self.current_session = TimerSession::Break; 
                            } else {
                                self.current_session = TimerSession::Resting;
                            }
                        }
                        TimerSession::Resting | TimerSession::Break => self.current_session = TimerSession::Working,
                    }
                    self.current_state = TimerState::CountDown
                }
                TimerState::Paused => {
                    // TODO: Resume - blocking input?
                    // TODO: Exit - blocking input?
                }
            }
        }
    }
}