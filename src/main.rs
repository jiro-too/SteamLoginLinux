#![allow(non_snake_case)]
#![allow(unused_parens)]
use std::{process::exit, string, path::Path, fs};

use text_io::{self, read};
use crate::file_helper::{check_exist, list_files_in_dir, del_files};
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

fn check_is_user_exists(path:String) -> bool{
    if check_exist("file".to_string(), path){
        return false;
    }
    else {
        return true;
    }
}

fn create_user(folder_path:String){
    let user_name : String = read!();
    let user_check = check_is_user_exists(folder_path.clone() + "/" + &user_name);
    if (user_check){
        let password  : String = read!();
        file_helper::create_file(folder_path.clone(), user_name.clone());
        let path_of_user = folder_path.clone() + "/" + &user_name.clone();
        file_helper::write_to_file(path_of_user, password);
    }
    else{
        panic!("USER ALREADY EXISTS!!!!");
    }
    return;
}

fn delete_user(folder_path:String){

    file_helper::list_files_in_dir(folder_path.clone()); // lists all the user present
    print!("Number of user to delete: ");
    let input :i32 = read!(); // user index to remove

    /*
    calling file_helper to read get the user names in a vector array
    proceeding to loop through the vector array while comparing the variable X with
    the specified input anc calling file_helper to delete the files of user achieved
    by appending user to the provided folder_path 
    */
    let string_vector = file_helper::read_dir_as_vec(folder_path.clone()); 
    let mut x = 1;
    for user in string_vector{
        let user_path = folder_path.clone()+"/" + &user;
        println!("USER PATH= {}",&user_path);
        if (input == x){
            del_files(user_path);
            break;
        }
        else{
            x+=1;
        }
    }
}
fn main() {
    // Using file helper module to check if config.ini exists, if not creating one and exiting.
    let file_check = file_helper::check_exist("file".to_string(), "config.ini".to_string());

    if (!file_check) {
        println!("config.ini file not found.\n Creating a skeleton configuration.");
        file_helper::create_file("".to_string(), "config.ini".to_string());
        // default config provided in ini_parse
        // automation of add++; object 
    }
    let mut ini_helper = ini_parse::ParsedDetails {
        folder_path: "".to_string(), // folder to save the login details
        steam_binary_location: "".to_string(), // folder of custom steam binary
    };

    // parsing the ini file, defined in ini_parse.rs
    ini_helper.parse_ini();

    /* cloning the folder_path from struct since the struct will be passed as an argument and borrow checker
    wont let an moved variable be used */

    let folder_path = ini_helper.folder_path.clone(); 
    create_storage_location(ini_helper); // create storage folders

    
    
    // main menu
    println!(" 1. Add an user\n 2. Remove users\n 3. List users\n 4. Select user");
    loop{
        println!("Enter a choice");
        let choice: String = read!(); 
        //if choice is 1 call the create user functions
        if choice == "1"{
            create_user(folder_path.clone());
        }
        else if (choice == "2"){
            delete_user(folder_path.clone());
        }
        else if choice == "3"{
            file_helper::list_files_in_dir(folder_path.clone());
        }
    }
    
}
