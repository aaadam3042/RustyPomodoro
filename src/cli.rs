use crate::timer::TimerEvent;
use crate::timer::TimerSession;
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
use crate::timer::TimerState;
use crate::utils::clear_terminal;
use crate::utils::start_input_thread;

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
            1 => cli_run_timer(app),
            2 => cli_edit_settings(app),
            3 => break,
            _ => unreachable!("User was somehow able to chose an invalid option"),
        };
    }
}

fn cli_run_timer(app: &mut PomodoroApp) {
    // When we start timer:
    //      Start app timer     
    //      Get timer info
    //      Display UI

    app.start_timer();
    let mut current_state = TimerState::Idle;
    let input_receiver = start_input_thread();
    loop {
        if let Some(event) = app.poll_timer_event() {
            current_state = event.state;
            if matches!(event.state, TimerState::Idle) {
                break;
            }

            display_timer(
                event.session, 
                event.state, 
                event.remaining, 
                event.cycles_complete
            );

        } else if app.is_timer_disconnected() {
            break;
        }

        // Handle input
        if let Ok(input) = input_receiver.try_recv() {
            handle_input(app, current_state, input);
        }

        std::thread::sleep(std::time::Duration::from_millis(30));
    }
}

fn display_timer(session: TimerSession, state: TimerState, time_remaining: u32, cycles: u32) {
        clear_terminal();
        println!("TIMER\n");

        println!("[{}]", state.as_str());
        println!("Session: {} \n", session.as_str());

        println!("Cycle {cycles}");
        println!("Time Remaining:");
        println!("{}",get_display_time(time_remaining));

        // Display the correct commands
        println!("{}", get_display_commands(state));
}

fn get_display_time(time_seconds: u32) -> String {
    // Rust int division always truncates
    format!("{}:{}", (time_seconds/60), time_seconds%60)
}

fn get_display_commands(state: TimerState) -> &'static str {
    match state {
        TimerState::CountDown => "Press 1 to Pause",
        TimerState::Paused => "Press 1 to Resume, Press 2 to Exit",
        TimerState::Waiting => "Press 1 to advance session, Press 2 to Exit",
        TimerState::Idle => "Returning to Main Menu"
    }
}

fn handle_input(app: &PomodoroApp, state: TimerState, input: u8) {
    match (state, input) {
        (TimerState::CountDown, 1) => app.pause_timer(),
        (TimerState::Paused, 1) => app.resume_timer(),
        (TimerState::Paused, 2) => app.stop_timer(),
        (TimerState::Waiting, 1) => app.advance_timer(),
        (TimerState::Waiting, 2) => app.stop_timer(),
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
