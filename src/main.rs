use colored::*;
mod utils;
mod helper;


#[allow(warnings)]

fn main() {

    let user_input:Vec<String> = std::env::args().collect();
    // Get filename
    let path:String = match user_input.get(1) {
        Some(value) => String::from(value),
        None => {
            let msg_ = "Please provide file path".red();
            println!("{}", msg_);
            String::new()
        }
    };
    let mut path_ ;
    if user_input.len() > 0 {
        path_ = user_input[1].clone()
    }

    let new_name:String = match user_input.get(3) {
        Some(value) => String::from(value),
        None => String::new()
    };
    
    if path == "help" || path ==  "--help"{
        utils::print_help();
    }
    else if path == "version" || path == "--version" || path == "--v"{
        utils::print_version();
    }
    else if user_input.len() >= 3{
        let res = nrt::get_filedetails_with_flag(path,user_input[2].clone(), new_name);
    }
    else {
        let read_ = nrt::read_file(path);
        println!("{}", read_);    
    }

}




