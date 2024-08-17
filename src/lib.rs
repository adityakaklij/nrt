
use colored::*;
use std::{fs::metadata, os::unix::fs::MetadataExt};
mod utils;
mod helper;
use chrono::{DateTime, Utc};

pub fn get_filedetails_with_flag(path_:String, mut flag: String, new_file_name_: String) -> std::io::Result<()> {

    helper::check_file_exists(path_.clone());
    let metadata = metadata(path_.clone())?;
    let flag_:String =   flag.split_off(2);

    if flag_ == "size" || flag_ == "s"{
        let len_in_bytes = metadata.len();
        println!("{:?} bytes", len_in_bytes); // Size in bytes
        println!("{:?} MB", len_in_bytes as f64/(1_000_000) as f64); // Size in MB
        if len_in_bytes > 1_00_000_000 {
            println!("{:?} GB", len_in_bytes as f64/(1_000_000_000) as f64); // Size in GB
        }
    }
    
    else if flag_ == "mt" {
        let last_modified_time = metadata.modified()?;
        let datetime: DateTime<Utc> = last_modified_time.into();
        let readable_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        println!("{} UTC", readable_time);
    }
    else if flag_ == "ct" { // File creation time
        let created_time = metadata.created()?;
        let datetime: DateTime<Utc> = created_time.into();
        let readable_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        println!("{} UTC", readable_time);
    }
    
    else if flag_ == "mode" { // 33188
        let file_mode = metadata.mode();
        println!("file_mode: {:?}", file_mode);
    }
    else if flag_ == "lc" || flag_ == "linecount" { // Return line count.
        let line_count = helper::calculate_line_of_code(path_).green();
        println!("Line count: {}", line_count);
    }
    else if flag_ == "rn" || flag_ == "rename" { // rename
        let res = helper::rename_file(path_, new_file_name_);
        println!("{}", res);
    }
    else if flag_ == "n" || flag_ == "number" { // Prints all nonempty lines with index
        let _res = helper::index_files(path_);
    }
    else if flag_ == "cp" || flag_ == "copy" { // Copy entire file content.
        let res = helper::copy_file_content(path_);
        println!("{}", res)
    }
    else {
        let cmd = "--help".blue();
        let msg1 = "use".green();
        let msg2 = "for more information".green();
        println!("{} {} {}", msg1, cmd, msg2);
    }

    // else if flag_ == "at" {
    //     let last_access_time = metadata.accessed()?;
    //     println!("access: {:?}", last_access_time); // Last access time
    // }

    // else if flag_ == "ft" {
    //     let file_type = metadata.file_type();
    //     println!("file_type: {:?}", file_type);
    // }

    Ok(())
}


pub fn read_file(path_:String) -> String {
    let file_staus = helper::check_file_exists(path_.clone());
    if file_staus {
        let content = std::fs::read_to_string(path_).expect("Unable to read file");
        if content.is_empty() {
            return String::from("File is empty");
        }
        content
    }
    else {
        String::new()
    }
}


