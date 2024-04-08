use std::{any::Any, io::{self, stdin, stdout, Read, Write}, process::{self, exit, Command, Stdio}, u8, usize};

use colored::Colorize;
use console::Term;
use crossterm::{execute, terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};



mod file_explorer;
mod config;

fn main() {
    let cfg = config::new();    
    let files = file_explorer::get_all_files(&cfg); 



    let mut capture_output = true;
    let term = Term::stdout();
    
    let mut input = String::new();


    let mut filtered_arr = filter(&files, &input);

    while capture_output {

        term.clear_screen().unwrap();

        let (row, _) = term.size();
        
        let entry_size = row - 2;
        filtered_arr = filter(&files, &input);
        let mut idx = 0;

        while idx < entry_size {
            let cursor = entry_size - idx;
            if usize::from(cursor) > filtered_arr.len()   {
                print!("\n");
            } else if filtered_arr.len() > 0{
                if cursor == 1 {
                    print!("> {} \n", &filtered_arr.get(usize::from(cursor - 1)).unwrap().green());
                } else {
                    print!("  {} \n", &filtered_arr.get(usize::from(cursor - 1)).unwrap());
                }
            }
            idx += 1; 
        }
        print!("  {}/{} \n", &filtered_arr.len(), files.len());
        print!("input: {} \n", input);



        let key = term.read_key().unwrap();
        match key {
            console::Key::Char(c) => {
                input = input + &c.to_string(); 
                filtered_arr = filter(&files, &input);
            } ,
            console::Key::Backspace => {
                if input.len() > 0 {
                    input.truncate(input.len() - 1);
                }
                filtered_arr = filter(&files, &input);
            },
            console::Key::Enter => {
                capture_output = false;
                execute!(stdout(), EnterAlternateScreen).unwrap();
                let dir = filtered_arr.get(0).unwrap();
                let tmux_session = Command::new("tmux")
                    .args(["new", "-A", "-s", dir.as_str(), "-c", dir.as_str()])
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .unwrap()
                    .wait();

                execute!(stdout(), LeaveAlternateScreen ).unwrap();

            },
            _ => (),
        }

    }

    print!("\x1B[2J\x1B[1;1H");
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
