use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;

/*
CLI will be called from main.rs
It will handle all the user interace (text prompts) allowing the user to:
- Start, pause and stop the timer
- Edit configuration data
It does not actually perform these actions, rather passes the instruction to the relevant service.
*/
use crate::utils;
use crate::PomodoroApp;
use crate::queryOptions;
use crate::timer::{TimerState, TimerSession};
use crate::utils::{clear_terminal, poll_user_input};
use crossterm::event::KeyCode::{self,Char};


pub fn run(app: &mut PomodoroApp) {
    loop {
        let settings = app.get_settings();

        utils::clear_terminal();

        println!("POMODORO TIMER\n");
        println!("Welcome to this pomodoro timer, modified for eye strain management.\n");
        println!("Your current settings are as such:");
        println!("{settings}\n");
        let option = queryOptions!("Options:","Start Timer", "Edit Settings", "Exit 🚪");

        match option {
            1 => {if let Err(_)  = cli_run_timer(app) {
                println!("Something went wrong when trying to start the timer. Try again!");
                println!("If the error persists try contacting an admin\n");
            }},
            2 => cli_edit_settings(app),
            3 => break,
            _ => unreachable!("User was somehow able to chose an invalid option"),
        };
    }
}

// Guard to automatically drop our raw mode when done 
struct RawModeGuard;
impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}

fn cli_run_timer(app: &mut PomodoroApp) -> Result<(), std::io::Error> {
    // When we start timer:
    //      Start app timer     
    //      Get timer info
    //      Display UI

    // For input 
    enable_raw_mode()?;
    let _guard = RawModeGuard;

    let total_cycles = app.get_settings().work_relief_cycles;
    app.start_timer();
    let mut current_state = TimerState::Idle;
    let mut has_drawn_waiting = false;

    loop {
        if let Some(event) = app.poll_timer_event() {
            current_state = event.state;
            if matches!(event.state, TimerState::Idle) {
                break;
            }

            // We dont want to draw multiple times if waiting
            if !(matches!(event.state, TimerState::Waiting | TimerState::Paused) && has_drawn_waiting) {
                has_drawn_waiting = true;
                display_timer(
                    event.session, 
                    event.state, 
                    event.remaining, 
                    event.cycles_complete,
                    total_cycles
                );
            }   
            if !matches!(event.state, TimerState::Waiting | TimerState::Paused) {has_drawn_waiting=false}

        } else if app.is_timer_disconnected() {
            break;
        }

        // Handle input
        if let Some(input) = poll_user_input() {
            handle_input(app, current_state, input);
        }

        std::thread::sleep(std::time::Duration::from_millis(30));
    } 

    Ok(())
}

fn display_timer(session: TimerSession, state: TimerState, time_remaining: u32, cycles: u32, total_cycles: u32) {
        clear_terminal();
        println!("TIMER\n");

        println!("[{}]", state.as_str());
        println!("Session: {} \n", session.as_str());

        println!("Cycle {}/{}", cycles+1, total_cycles);
        println!("Time Remaining:");
        println!("{}",get_display_time(time_remaining));

        // Display the correct commands
        println!("{} (Then Enter to submit command)", get_display_commands(state));  
}

fn get_display_time(time_seconds: u32) -> String {
    // Rust int division always truncates
    format!("{:02}:{:02}", (time_seconds/60), time_seconds%60)
}

fn get_display_commands(state: TimerState) -> &'static str {
    match state {
        TimerState::CountDown => "Press 1 to Pause",
        TimerState::Paused => "Press 1 to Resume, Press 2 to Exit",
        TimerState::Waiting => "Press 1 to advance session, Press 2 to Exit",
        TimerState::Idle => "Returning to Main Menu"
    }
}

fn handle_input(app: &PomodoroApp, state: TimerState, input: KeyCode) {
    match (state, input) {
        (TimerState::CountDown, Char('1')) => app.pause_timer(),
        (TimerState::Paused, Char('1')) => app.resume_timer(),
        (TimerState::Paused, Char('2')) => app.stop_timer(),
        (TimerState::Waiting, Char('1')) => app.advance_timer(),
        (TimerState::Waiting, Char('2')) => app.stop_timer(),
        _ => {/* Do Nothing if unrecognised command */}
    }
}

fn cli_edit_settings(app: &mut PomodoroApp) {
    // When we edit settings:
    //      Display UI
    //      Wait for user input -> multiple menus
    //      call app to save        <APP
    //      Go back to main menu
    let mut new_settings = app.get_settings().clone();

    loop {
        utils::clear_terminal();
        println!("CONFIGURE SETTINGS\n");

        println!("Your current settings are as such:");
        println!("{new_settings}\n");

        let option = queryOptions!("Options:", "Work time", "Relief time", "Break time", "no. Cycles", "Save and Exit 💾", "Back 🚪");
        match option {
            1 => new_settings.work_seconds = utils::get_posint_input("\nSet work timer in minutes:") * 60,
            2 => new_settings.relief_seconds = utils::get_posint_input("\nSet relief timer in seconds:"),
            3 => new_settings.break_seconds = utils::get_posint_input("\nSet break timer in minutes:") * 60,
            4 => new_settings.work_relief_cycles = utils::get_posint_input("\nSet number of cycles (no. work-relief sessions before break):"),
            5 => {
                // Save and Exit option
                app.save_config(new_settings);
                break;
            },
            6 => break,
            _ => unreachable!("User was somehow able to chose an invalid option"),
        }
    }
}
