use colored::*;
use std::fs::File;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use arboard::Clipboard;

// Counts total number of lines. (inclusive of empty lines)
// fn calculate_line_of_code(path_:String) -> io::Result<()> {
#[allow(warnings)]
pub fn calculate_line_of_code(path_:String) -> String {
        let file = File::open(path_).unwrap();    
        let reader = io::BufReader::new(file);
        let line_count = reader.lines().count();
        line_count.to_string()
    }
#[allow(warnings)]
pub fn rename_file(path_: String, new_file_name_ : String) -> String{
        let p = fs::rename(path_, new_file_name_);
        let success = "Successfully renamed file";
        let err = "Unable to rename file";
        match p {
            Ok(_p) => success.green().to_string(),
            Err(e) => {
                // Print the error message
                println!("Error: {:?}", e);
                // Return the error message
                format!("{}: {}", err.red(), e)
            },
            // Err(e) => err.red().to_string()
        }
    }
    #[allow(warnings)]
    pub fn check_file_exists(path_:String) -> bool {
        if Path::new(&path_).exists(){
            true
        }
        else {
            let str_ = "File does not exist";
            println!("{}", str_.red());
            false
        }
    }
    
    // Returns each line with indexing.
    #[allow(warnings)]
    pub fn index_files(path_:String){


    let content = std::fs::read_to_string(&path_).expect("Unable to read file");
    if content.is_empty() { 
        let res = "File is empty".red();
        println!("{}", res);
    }

    let file = File::open(&path_);
    
    let reader = io::BufReader::new(file.unwrap());
    let mut line_counter = 1;
    for i in reader.lines() {
        let str_ = i.unwrap();
        if str_.clone().is_empty(){
            continue;
        }
        println!("{} {}" ,line_counter.to_string().blue(),  str_.to_string());
        line_counter += 1;
    }   
}

#[allow(warnings)]
pub fn copy_file_content(path_: String) -> String{

    let file_staus = check_file_exists(path_.clone());
    if file_staus {
        let content = std::fs::read_to_string(path_).expect("Unable to read file");
        if content.is_empty() {
            let msg = "File is empty".red();
            return msg.to_string()
        }
        let mut clipboard = Clipboard::new().unwrap();
        // Copy content to clipboard
        clipboard.set_text(content).unwrap();
        let msg = "Text copied successfully".green();
        return msg.to_string();
    }
    else {
        String::new()
    }
}