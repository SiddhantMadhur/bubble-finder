use std::{any::Any, io::{stdin, stdout, Read, Write}, process::{self, exit, Command}, u8, usize};

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

        term.clear_screen();

        let (row, _) = term.size();
        
        let entrySize = row - 1;
        filtered_arr = filter(&files, &input);
        let mut idx = 0;

        while idx < entrySize {
            let cursor = entrySize - idx;
            if usize::from(cursor) > filtered_arr.len()   {
                print!("\n");
            } else if filtered_arr.len() > 0{
                print!("> {} \n", filtered_arr.get(usize::from(cursor - 1)).unwrap());
            }
            idx += 1; 
        }
        print!("input: {} \n", input);


        let mut cur = 0;


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
                Command::new("tmux")
                    .arg("new")
                    .spawn()
                    .unwrap();
                    
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
