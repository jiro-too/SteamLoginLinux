#![allow(non_snake_case)]
#![allow(unused_parens)]
use std::process::exit;

use text_io::{self, read};
use crate::file_helper::check_exist;
mod file_helper;
mod ini_parse;

fn create_storage_location(ini_helper: ini_parse::ParsedDetails){
    /*Create folder to store the login directories
    If the specified folder does not exist creates the folder calling file_helper otherwise
    returns without doing anything. To be optimized */
    let folder_path = ini_helper.folder_path.clone();
    let check_custom_dir_exist = check_exist("dir".to_string(), folder_path.clone());
    if(!check_custom_dir_exist){
        file_helper::create_directory(folder_path.clone()).expect("Error");
    }
    return;
}

fn main() {
    // Using file helper module to check if config.ini exists, if not creating one and exiting.
    let file_check = file_helper::check_exist("file".to_string(), "config.ini".to_string());

    if (!file_check) {
        println!("config.ini file not found.\n Creating a skeleton configuration.");
        file_helper::create_file("".to_string(), "config.ini".to_string());
        // default config provided in ini_parse
        // automation of adding default values to ini planned
        println!("Configure the ini file and restart the program");
        exit(1);
    }
    //creating the ini_helper object 
    let mut ini_helper = ini_parse::ParsedDetails {
        folder_path: "".to_string(), // folder to save the login details
        steam_binary_location: "".to_string(), // folder of custom steam binary
    };

    // parsing the ini file, defined in ini_parse.rs
    ini_helper.parse_ini();

    let folder_path = ini_helper.folder_path.clone(); 
    /* cloning the folder_path from struct since the struct will be passed as an argument and borrow checker
    wont let an moved variable be used */
    create_storage_location(ini_helper); // create storage folders
    println!(" 1. Add an user\n 2.Remove users\n 3. List users");
    loop{
        println!("Enter a choice");
        let choice: String = read!();
        if choice == "1"{
            let user_name : String = read!();
            let password  : String = read!();
            file_helper::create_file(folder_path.clone(), user_name.clone());
            let path_of_user = folder_path.clone() + "/" + &user_name.clone();
            file_helper::write_to_file(path_of_user, password);
        }
        else if choice == "3"{
            file_helper::list_files_in_dir(folder_path.clone());
        }
    }
    
}
