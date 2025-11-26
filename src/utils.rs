use std::error::Error;
use std::io::{self, Write};
use std::time::Duration;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};

#[macro_export]
macro_rules! queryOptions {
    ($prompt:expr, $($opt:expr),+ $(,)?) => {{
        let options = vec![$($opt),+];
        utils::query_user_option($prompt, &options)
    }};
}

pub fn query_user_option(prompt: &str, options: &[&str]) -> u8 {
    let num_options = options.len() as u8;

    println!("{prompt}");
    for (i, option) in options.iter().enumerate() {
        println!("{}: {}", i+1, option);
    }

    let mut is_valid_option = false;
    let mut option = 0;
    while !is_valid_option {
        option =  match get_input_option(num_options) {
            Ok(v) => {
                is_valid_option = true;
                v
            }
            Err(_) => {
                println!("Please input a valid option!");
                continue
            }
        }
    }
    option
}

fn get_input_option(num_options: u8) -> Result<u8, Box<dyn Error>> {
    print!("\nSelect an option: \n> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input)?; 
    let option = input.trim().parse::<u8>()?;
    (1..=num_options)
        .contains(&option)
        .then_some(option)
        .ok_or_else(|| From::from("Invalid option selected")) 
}

pub fn get_input(s1: &str) -> String {
    print!("{}\n> ", s1);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn get_posint_input(s1: &str) -> u32 {
    loop {  
        let rs = get_input(s1).parse::<u32>();
        match rs {
            Ok(v) => if v > 0 {return v} ,
            Err(_) => ()
        }
        println!("Please enter a valid integer > 0!")
    }
}

pub fn clear_terminal() {
    print!("{}[2J", 27 as char); 
    io::stdout().flush().unwrap();          
}

pub fn poll_user_input() -> Option<KeyCode> {
    if poll(Duration::from_millis(0)).ok()? {
        if let Event::Key(KeyEvent {code, .. }) =  read().ok()? {
            return Some(code);
        } 
    } 
    None
}