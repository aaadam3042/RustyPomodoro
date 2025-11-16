mod app;
mod cli;
mod config_manager;
mod timer;
mod utils;
use std::env;

use crate::app::PomodoroApp;

fn main() -> Result<(), String> {
    // Just parse arguments here if any (For later GUI implement)
    // Then launch CLI or GUI
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 1 {
        println!("Incorrect usage! Please run without any arguments.");
    } else {
        let mut app = PomodoroApp::new();
        app.init();
        cli::run(&mut app);
    }
    Ok(())
}
