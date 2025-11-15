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

pub fn run(app: &PomodoroApp) {
    let settings = app.get_settings();

    utils::clear_terminal();

    println!("POMODORO TIMER\n");
    println!("Welcome to this pomodoro timer, modified for eye strain management.");
    println!("Your current settings are as such:");
    println!("{settings}");
    let option = queryOptions!("Options:","Start Timer", "Edit Settings");

    // TODO: I think while query options happens here, this logic should happen in app as something like run_option(), which essentially handles state
    match option {
        1 => {
            show_start_timer();
        }
        2 => {
            show_edit_settings();
        }
        _ => {
            unreachable!("User was somehow able to chose an invalid option");
        }
    }

    // TODO: We want enums for each state, map numbers to return each state, then run a state machine -> this goes in app??
    // How far do we go into app? Should there be any match here at all. Should we occasionally query app for the current state to display to
    // CLI. Or do we do like above and then somehow integrate app only for timer, and saving and loading??
}

fn show_start_timer() {

}

fn show_edit_settings() {

}
