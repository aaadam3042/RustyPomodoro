use std::error::Error;
use std::io::{self, Write};

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
    println!("Select an option: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let option = input.parse::<u8>()?;
    (1..=num_options)
        .contains(&option)
        .then_some(option)
        .ok_or_else(|| From::from("Invalid option selected"))
    
}

pub fn get_input(s1: String) -> String {
    print!("{}", s1);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn clear_terminal() {
    print!("{}[2J", 27 as char); 
    io::stdout().flush().unwrap();          
}