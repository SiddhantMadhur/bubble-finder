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


    let mut filtered_arr = filter(&files, &input);
    let mut highlighted_index = filtered_arr.len()-1;

    loop {
        print!("{}[2J", 27 as char);

        filtered_arr = filter(&files, &input);
        let mut cur = 0;
        for file in &filtered_arr  {
            if cur == highlighted_index {
                println!("{}{}", " > ".green() ,&file.green());
            } else {
                println!(" > {}", &file);
            }
            cur += 1;
        }

        println!("\n ({}/{})\n\n > {} ", &filtered_arr.len(), &files.len(), &input);

        let key = term.read_key().unwrap();
        match key {
            console::Key::Char(c) => {
                input = input + &c.to_string(); 
                filtered_arr = filter(&files, &input);
                highlighted_index = filtered_arr.len()-1;
            } ,
            console::Key::Backspace => {
                if input.len() > 0 {
                    input.truncate(input.len() - 1);
                }
                filtered_arr = filter(&files, &input);
                highlighted_index = filtered_arr.len()-1;
            },
            console::Key::Enter => {
                exit(1);
            },
            console::Key::ArrowUp => {
                highlighted_index -= 1;
            },
            console::Key::ArrowDown => {
                highlighted_index += 1;
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
