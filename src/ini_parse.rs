extern crate ini;

use ini::Ini;

pub struct ParsedDetails{
    pub folder_path : String,
    pub steam_binary_location : String,  
}

impl ParsedDetails {
    pub fn parse_ini(&mut self){
        /*
        ini configuration consists of the folder to save login details and
        a custom steam binary location if provided, if no config.ini exists
        it will be created in main.rs
        
        Example config.ini:

        [config]
        folder_path=default
        steam_binary_path=default
        */
        let ini_conf = Ini::load_from_file("config.ini").unwrap();
        // loads the [config] section from the ini
        let section = ini_conf.section(Some("config")).unwrap();
        // loads the folder_path
        let folder_path = section.get("folder_path").unwrap();
        // removed after test println!(self.folder_path)
        

        if (folder_path == "default"){
            self.folder_path = "./SteamLogin".to_string();
        }
        else{
            self.folder_path = folder_path.to_string();
        }
    }
}
