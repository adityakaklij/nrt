use colored::*;

#[allow(warnings)]
pub fn print_version() {

    let title = "version 0.1.0".green().bold();

    let version = 
        "Copyright (C) 2024 Aditya K.
License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.
This is free software.
There is NO WARRANTY, to the extent permitted by law.
by Aditya K.".blue();

    println!();
    println!("{title}");
    println!();
    println!("{version}");
    println!();
}
#[allow(warnings)]
pub fn print_help() {
    
    let help_text = format!(r#"

Usage: {}

Options:

  {}     Size of the file in bytes.
  {}       Last modified time in UTC.
  {}       Creation time in UTC.
  {}     File mode (permissions) in numeric form.
  {}       Number of lines in the file.
  {}        Print all non-empty lines with their index.
  {}       Copy eniter file content.
  {}        Version.
  {}Rename the file to a new filename.
  {}     Get help.
    "#, 
    "nrt <filename> [OPTION]".cyan(),
    "--size".cyan(),
    "--mt".cyan(),
    "--ct".cyan(),
    "--mode".cyan(),
    "--lc".cyan(),
    "--n".cyan(),
    "--cp".cyan(),
    "--v".cyan(),
    "--rn <new_filename> ".cyan(),
    "--help".yellow(),

);

    println!("{}", help_text);

    // println!("{}", p);
}
