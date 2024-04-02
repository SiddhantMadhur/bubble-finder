use std::{any::Any, io::{stdin, stdout, Read, Write}, process::exit, u8};

use colored::Colorize;
use console::Term;





mod file_explorer;
mod config;

fn main() {
    let cfg = config::new();    
    let files = file_explorer::get_all_files(&cfg); 



    let term = Term::stdout();
    
    let mut input = String::new();

    loop {
        print!("{}[2J", 27 as char);

        let filtered_arr = filter(&files, &input);
        for file in &filtered_arr  {
            if file == filtered_arr.last().unwrap() {
                println!("{}{}", " > ".green() ,&file.green());
            } else {
                println!(" > {}", &file);
            }
        }

        println!("\n ({}/{})\n\n > {} ", &filtered_arr.len(), &files.len(), &input);

        let key = term.read_key().unwrap();
        match key {
            console::Key::Char(c) => {
                input = input + &c.to_string(); 
            } ,
            console::Key::Backspace => {
                if input.len() > 0 {
                    input.truncate(input.len() - 1);
                }
            },
            console::Key::Enter => {
                exit(1);
            },
            _ => (),
        }


    }


}



fn filter(arr: &Vec<String>, filter_str: &String) -> Vec<String> {
    let mut proc: Vec<String> = vec![];
    for i in arr.iter() {
        if i.contains(filter_str) {
            proc.push(i.to_string());
        }
    }
    proc
}
