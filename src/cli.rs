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
            1 => cli_run_timer(),
            2 => cli_edit_settings(app),
            3 => break,
            _ => unreachable!("User was somehow able to chose an invalid option"),
        };
    }
}

fn cli_run_timer() {
    // When we start timer:
    //      Display UI
    //      Start app timer     <APP
    //      Update UI       <Listener?
    loop {
        println!("TIMER\n");

        // println!("Session: {} \n");

        println!("Time Remaining:")
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
