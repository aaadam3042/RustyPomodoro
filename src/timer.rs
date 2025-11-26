use crate::config_manager::Settings;

use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use std::time::Duration;

pub enum TimerCommand {
    Pause, 
    Resume, 
    Stop,
    Next,
}

pub struct TimerEvent {
    pub state: TimerState,
    pub session: TimerSession,
    pub remaining: u32,
    pub cycles_complete: u32,
}

pub struct TimerHandle {
    pub cmd_tx: Sender<TimerCommand>,
    pub evt_rx: Receiver<TimerEvent>,
}

// Enum to keep timer states
#[derive(Copy, Clone)]
pub enum TimerState {
    Idle,
    CountDown,
    Waiting,
    Paused,
}

impl TimerState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Idle => "Idle",
            Self::CountDown => "Count Down",
            Self::Waiting => "Waiting",
            Self::Paused => "Paused",
        }
    }
}

#[derive(Copy, Clone)]
pub enum TimerSession {
    Working,
    Resting,
    Break
}

impl TimerSession {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Working => "Working",
            Self::Resting => "Resting",
            Self::Break => "Break"
        }
    }
}

pub struct Timer {
    timer_settings: Settings,
    current_state: TimerState,
    current_session: TimerSession,
    time_remaining: u32,
    cycles_complete: u32,
}

impl Timer {
    pub fn new(settings: Settings) -> Self {
        Self {
            timer_settings: settings,
            current_state: TimerState::Idle,
            current_session: TimerSession::Working,
            time_remaining: 0,
            cycles_complete: 0,
        }
    }

    fn prepare_start(&mut self) {
        self.current_state = TimerState::CountDown;
        self.current_session = TimerSession::Working;
        self.time_remaining = self.timer_settings.work_seconds;
        self.cycles_complete = 0;
    }

    fn tick(&mut self) {
        if let TimerState::CountDown = self.current_state {
            if self.time_remaining > 0 {
                self.time_remaining -= 1;
            } else {
                self.current_state = TimerState::Waiting;
                    // TODO: Play Audio on interval until input - non blocking? - needs to loop? remember tick is only continously
                    // Called if in countdown, which we wont be here
            }
        }
    }

    fn next_session(&mut self) {
        if let TimerState::Waiting = self.current_state {
            match self.current_session {
                TimerSession::Working => {
                    if self.cycles_complete == self.timer_settings.work_relief_cycles-1 {
                        self.current_session = TimerSession::Break;
                        self.time_remaining = self.timer_settings.break_seconds;
                    } else {
                        self.current_session = TimerSession::Resting;
                        self.time_remaining = self.timer_settings.relief_seconds;
                    }
                }
                TimerSession::Break | TimerSession::Resting => {
                    if matches!(self.current_session, TimerSession::Break) {self.cycles_complete = 0}
                    else {self.cycles_complete += 1}
                    self.current_session = TimerSession::Working;
                    self.time_remaining = self.timer_settings.work_seconds;
                }
            }
            self.current_state = TimerState::CountDown;
        }
    }

    fn pause(&mut self) {
        if let TimerState::CountDown = self.current_state {
            self.current_state = TimerState::Paused;
        }
    }

    fn resume(&mut self) {
        if let TimerState::Paused = self.current_state {
            self.current_state = TimerState::CountDown;
        }
    }

    fn stop(&mut self) -> bool {
        // We only stop if paused or waiting
        match self.current_state {
            TimerState::Paused | TimerState::Waiting => {
                self.current_state = TimerState::Idle;
                self.time_remaining = 0;
                self.cycles_complete = 0;
                true
            }
            _ => false
        }   
    }

    fn get_state(&self) -> TimerState {
        self.current_state
    }

    fn get_session(&self) -> TimerSession {
        self.current_session
    }

    fn get_remaining(&self) -> u32 {
        self.time_remaining
    }

    fn get_cycles_complete(&self) -> u32 {
        self.cycles_complete
    }

    pub fn spawn(settings: Settings) -> TimerHandle {
        let (cmd_tx, cmd_rx) = mpsc::channel::<TimerCommand>();
        let (evt_tx, evt_rx) = mpsc::channel::<TimerEvent>();

        thread::spawn(move || {
            let mut timer = Timer::new(settings);
            timer.prepare_start();

            loop {
                // 1. Handle commands if any
                if let Ok(cmd) = cmd_rx.try_recv() {
                    match cmd {
                        TimerCommand::Pause => timer.pause(),
                        TimerCommand::Resume => timer.resume(),
                        TimerCommand::Next => timer.next_session(),
                        TimerCommand::Stop => {
                            if timer.stop() {
                                break;
                            }
                        },
                    }
                }

                // 2. Tick if counting down
                if let TimerState::CountDown = timer.get_state() {
                    timer.tick();
                }

                // send status (best-effort)
                let _ = evt_tx.send(TimerEvent {
                    state: timer.get_state(),  
                    session: timer.get_session(),
                    remaining: timer.get_remaining(),
                    cycles_complete: timer.get_cycles_complete(),
                });

                thread::sleep(Duration::from_secs(1));
            }
        });

        TimerHandle { cmd_tx, evt_rx }
    }
}