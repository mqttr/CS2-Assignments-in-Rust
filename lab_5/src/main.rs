use regex::Regex;
use text_io::read;

use std::fs;
use std:: {path::Path, path::PathBuf};
use std::{io, io::Write};
use std::error;

fn main() {
    let root_dir: PathBuf = match Path::new(r"./files/").canonicalize() {
        Ok(path) => path,
        Err(_) => {
            println!("Cannot find absolute path to relative directory: './files/'. Does './files/' exist?");
            std::process::exit(1);
        }
    };

    let infile = get_infile(&root_dir);

    let outfile = match get_outfile(&root_dir) {
        Ok(path) => path,
        Err(_) => {
            println!("Error in getting outfile. This shoud not happen. Please contact dev.");
            std::process::exit(1);
        }
    };

    println!("{}", outfile.display());
}

fn get_infile(root_path: &Path) -> Result<PathBuf, std::io::Error> {
    Ok(PathBuf::from("jgiaegi.txt"))
}

fn get_outfile(root_path: &Path) -> Result<PathBuf, std::io::Error> {
    
    let mut user_tmp: String;
    let mut test_tmp: String;


    loop {
        print!("Output File Name, include extension: ");
        io::stdout().flush().unwrap();

        user_tmp = read!();

        let path_tmp: PathBuf = root_path.join(PathBuf::from(user_tmp.trim()));
        
        if path_tmp.exists() {
            let r = loop {
                print!("'{:#?}' already exists would you like to replace it? (y/n): ", path_tmp);
                io::stdout().flush().unwrap();
    
                test_tmp = read!();

                match test_tmp.to_lowercase().trim() {
                    "y" => break true,
                    "n" => break false,
                    _ => continue
                }
            };

            if r == true {
                return Ok(path_tmp);
            } else {
                continue;
            }


        } else {
            return Ok(path_tmp);
        }
    };
}