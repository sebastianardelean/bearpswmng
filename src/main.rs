//pub mod cli;
use log::{info,trace,warn,error};
use log4rs;

//use crate::cli::cli::execute
use home;
use std::path::{Path, PathBuf};
use std::fs;

fn main() {
    const CONFIG_DIRECTORY_NAME: &str = ".bearpswmng";
    
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    trace!("Trying to get the home directory!");

    let mut home_directory =
        home::home_dir().unwrap_or_else(|| {
            error!("Impossible to get home directory");
            PathBuf::from("/")
            });

    trace!("Home directory path: {}", home_directory.display());
    home_directory.push(CONFIG_DIRECTORY_NAME);
    trace!("Config directory path: {}", home_directory.display());    



    match create_directory_if_missing(home_directory.as_path()) {
        Some(_) => {
            trace!("Directory check/create succeeded");
        }
        None => {
            error!("Directory check/create failed");
        }
    }
}


fn create_directory_if_missing(home_path:&Path) -> Option<()>{
    match fs::metadata(home_path) {
        Ok(meta) if meta.is_dir() => {
            trace!("Directory {} already exists!", home_path.display());
            Some(())
        }
        Ok(_) => {
            error!("{} exists but is not a directory!", home_path.display());
            None
        }
        Err(_) => {
            trace!("Try to create directory {}", home_path.display());
            match fs::create_dir(home_path) {
                Ok(_) => {
                    trace!("Directory {} created successfully", home_path.display());
                    Some(())
                }
                Err(e) => {
                    error!("Error creating directory {}", e);
                    None
                }
            }
        }
    }
}


