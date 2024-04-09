use std::{any::Any, env::args, io::{stdout, Write}, process::{Command, Stdio}, u8, usize};

use colored::Colorize;
use console::Term;
use crossterm::{cursor::{Hide, Show}, execute, queue, style::{Print, PrintStyledContent, Stylize}, terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand, QueueableCommand};



mod file_explorer;
mod config;

fn main() {

    let args = args();
    if args.len() > 1 {
        let arg = args.into_iter().last().unwrap(); 
        match arg.as_str() {
            "-v" => println!("Straw\nVersion: 2.0-alpha"),
            _ => println!("Argument {} not recognized.", arg) 
        }
        std::process::exit(0);
    }
    let cfg = config::new();    
    let files = file_explorer::get_all_files(&cfg); 


    let mut capture_output = true;
    let mut term = Term::stdout();
    
    let mut input = String::new();

    let mut user_cursor = 0;

    let mut filtered_arr = filter(&files, &input);

    while capture_output {

        term.flush().unwrap();

        let (row, _) = term.size();
        
        let entry_size = row - 4;
        filtered_arr = filter(&files, &input);
        let mut idx = 0;
        let mut output = String::from("");
        while idx < entry_size  {
            let cursor = entry_size - idx;
            if usize::from(cursor) > filtered_arr.len()   {
                output += format!("\n").as_str();
            } else if filtered_arr.len() > 0{
                if cursor == user_cursor + 1 {
                    output = output + format!("> {} \n", &filtered_arr.get(usize::from(cursor - 1)).unwrap().clone().green()).as_str();
                } else {
                    output = output + (format!("  {} \n", &filtered_arr.get(usize::from(cursor - 1)).unwrap())).as_str();
                }
            }
            idx += 1; 
        }
        output += format!("\n").as_str();
        output += format!("  {}/{} \n", &filtered_arr.len(), files.len()).as_str();
        output += format!("Search: {} \n",  input).as_str();
        
        term.execute(Print(output)).unwrap();

        let key = term.read_key().unwrap();
        match key {
            console::Key::Char(c) => {
                input = input + &c.to_string(); 
                filtered_arr = filter(&files, &input);
                user_cursor = 0;
            } ,
            console::Key::Backspace => {
                if input.len() > 0 {
                    input.truncate(input.len() - 1);
                }
                filtered_arr = filter(&files, &input);
                user_cursor = 0;
            },
            console::Key::Enter => {
                capture_output = false;
                execute!(stdout(), EnterAlternateScreen).unwrap();
                let dir: &String = filtered_arr.get(usize::from(user_cursor)).unwrap();
                let tmux_session = Command::new("tmux")
                    .args(["new", "-A", "-s", dir.as_str(), "-c", dir.as_str()])
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .unwrap()
                    .wait();

                execute!(stdout(), LeaveAlternateScreen ).unwrap();

            },
            console::Key::ArrowDown => {
                if user_cursor > 0 {
                    user_cursor -= 1;
                }

            },
            console::Key::ArrowUp => {
                if (user_cursor < entry_size-1) && usize::from(user_cursor) < filtered_arr.len()-1 {
                    user_cursor += 1;
                }
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
