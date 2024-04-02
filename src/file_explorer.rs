use std::str::FromStr;

use walkdir::WalkDir;

use crate::config::Config;


fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    let file_name = entry.file_name().to_str().expect("Could not convert");
    file_name.starts_with(".") && file_name != ".git"
}


pub fn get_all_files(config: &Config) -> Vec<String> {

    let mut git_projects = vec![];
    
    for dir in config.directories.iter() {
        for entry in WalkDir::new(dir).into_iter().filter_entry(|e| !is_hidden(e)) {
            match entry {
                Ok(d) => {
                    if (&d).file_name().to_str().expect("Error in converting to string") == ".git" {
                        git_projects.insert(
                            0, 
                            String::from_str(
                                d.path()
                                .to_str()
                                .expect("Not str")
                                )
                            .expect("Could not convert")
                            .replace("/.git", "")
                            );
                    }
                },
                Err(_) => () 
            }
        };
    }

    git_projects
}
