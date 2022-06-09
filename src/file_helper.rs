#[allow(unused_imports)]
use std::fs::{self, File};
use std::{io::Write, fs::OpenOptions};


pub fn read_dir_as_vec(path:String) -> Vec<String>{
    // Reads the file in specified folder and returns without path
    let string_vector: Vec<String> = std::fs::read_dir(path.clone()).unwrap()
    .map(|dir_entry| dir_entry.unwrap().path().file_name().unwrap().to_str().unwrap().to_owned())
    .collect();
    return string_vector;
}

pub fn write_to_file(path:String,buffer:String){
    //opens file at specified path with read-write permissions
    let mut file = OpenOptions::new()
                                  .write(true)
                                  .read(true)
                                  .truncate(true)
                                  .open(path)
                                  .unwrap();
    //write specified buffer to provided path                           
    file.write_all(buffer.as_bytes()).expect("Error writing to the file");
}

pub fn check_exist(type_to_check:String, path:String) -> bool{
    /*
    Checks whether an directory or file exists. Essentially a wrapper to make working with std::Path less
    nasty. 
    */
    if type_to_check == "file"{
        let file = std::path::Path::new(&path).exists();
        return file;
    }
    if type_to_check == "dir"{
        let is_dir  = std::path::Path::new(&path).is_dir();
        return is_dir;
    }
    else{
        // If no type is provided explicitely is provided panic.
        panic!("NO TYPE PROVIDED");
    }
}

pub fn create_directory(path:String) -> std::io::Result<()>{
    // No explanation of what this does; should be self explanatory , refer to rust docs and
    // https://www.codegrepper.com/code-examples/rust/rust+create+directory+if+not+exists
    fs::create_dir(path)?;
    println!("Directory created successfully\n");
    Ok(()) 
}

pub fn del_files(path:String){
    fs::remove_file(path).expect("Error in deleting the file");
}

pub fn list_files_in_dir(path:String){
    /*
    Lists all the files in a directory. Since the steam usernames are stored as the file-names in this directory
    the user variable can be later be passed to the login function or something of that sort. Currently remains as a mean
    to list the saved users to the user.
    */
    let mut count:i32 = 0;
    let files = read_dir_as_vec(path.clone());
    for users in files{
        println!("{}. {}",count+1,users);
        count+=1;
    } // fancy shit to display user number , username
        /* example
        0. Chal
        1. LOL 
        */
}

pub fn create_file(path:String , file_name:String){
    let file:String;
    if (path == ""){
        file = file_name;
    }
    else{
        file = path + "/"+ &file_name;
    }    
    println!("{}",file);
    File::create(file).expect("Error Creating the file!!!!");
}

