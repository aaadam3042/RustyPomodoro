/*
CLI will be called from main.rs
It will handle all the user interace (text prompts) allowing the user to:
- Start, pause and stop the timer
- Edit configuration data
It does not actually perform these actions, rather passes the instruction to the relevant service.
*/

use crate::utils;
use crate::PomodoroApp;

pub fn run(app: &PomodoroApp) {
    let settings = app.get_settings();

    utils::clear_terminal();

    println!("POMODORO TIMER");
    println!("Welcome to this pomodoro timer, modified for eye strain management.");
    println!("Your current settings are as such:");
    println!("{} minutes, {} seconds, {} minutes, for {} cycles", settings.);
}