mod cli;
use log::{info,trace,warn,error};
use log4rs;


use home;
use std::path::{Path, PathBuf};
use std::{fs,io};
use clap::Parser;
use cli::{CliArgs, Commands};
const CONFIG_DIRECTORY_NAME: &str = ".bearpswmng";

fn main() -> io::Result<()>{

    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    trace!("Trying to get the home directory!");

    let home_directory =
        home::home_dir().unwrap_or_else(|| {
            error!("Impossible to get home directory");
            PathBuf::from("/")
        });

    trace!("Home directory path: {}", home_directory.display());

    let config_directory = home_directory.join(CONFIG_DIRECTORY_NAME);

    trace!("Config directory path: {}", config_directory.display());

    
    create_directory_if_missing(config_directory.as_path());

    trace!("Reading arguments!");
    
    let args = CliArgs::parse();

    match &args.command {
        Commands::List => {
            trace!("Read directory {}", config_directory.display());
            match read_dirs(config_directory.as_path()) {
                Ok(groups) => {
                    for e in groups {
                        trace!("Entry: {}",e.display());
                    }
                }
                Err(e) => error!("Error reading directories: {}",e)
            }

        },

        Commands::Show(arg) => {
            let record_dir = config_directory.join(arg.name.clone());
            trace!("Let's check if {} exists", record_dir.display());
            match is_directory_missing(record_dir.as_path()) {
                Ok(false) => trace!("Exists!"),
                Ok(true) => trace!("Missing!"),
                Err(e) => error!("Error {}!", e),
            }
            
        },

        Commands::Add(arg) => {
            trace!("Add record {}; Jump to interactive mode", arg.name);
        }
    }

    


    Ok(())



   



  






    
}




fn is_directory_missing(dir_path: &Path) -> io::Result<bool> {
    match fs::metadata(dir_path) {
        Ok(meta) => {
            if meta.is_dir() {
                Ok(false)
            }
            else {
                Ok(true)
            }
        },
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(true),
        Err(e) => Err(e), //the real error
    }
}

fn create_directory_if_missing(home_path:&Path){
    match is_directory_missing(home_path) {
        Ok(false) => trace!("Directory {} already exists!", home_path.display()),
        Ok(true) => {
            trace!("Try to create directory {}", home_path.display());
            match fs::create_dir(home_path) {
                Ok(_) => {
                    trace!("Directory {} created successfully", home_path.display());
                    
                }
                Err(e) => {
                    error!("Error creating directory {}", e);
                    
                }
            }

        },
        Err(e) => error!("Error checking if directory is missing: {}",e),
    }
    

}

fn read_dirs(config_path: &Path) -> io::Result<Vec<PathBuf>>{
     let mut dir_entries = fs::read_dir(config_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    dir_entries.sort();
    Ok(dir_entries)
}
