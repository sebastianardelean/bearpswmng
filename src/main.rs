mod cli;
mod bearcrypto;
use log::{info,trace,warn,error};
use log4rs;


use home;
use std::path::{Path, PathBuf};
use std::{fs,io};
use std::fs::File;
use std::io::{Write, BufRead,Read};
use clap::Parser;
use cli::{CliArgs, Commands};



use bearcrypto::{encrypt_large_file, decrypt_large_file, encrypt_data,decrypt_data};





const CONFIG_DIRECTORY_NAME: &str = ".bearpswmng";

struct RecordData {
    username: String,
    password: String,
    other_info: Vec<String>    
}




fn main() -> io::Result<()>{

    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    trace!("Trying to get the home directory!");

    let home_directory: PathBuf =
        home::home_dir().unwrap_or_else(|| {
            error!("Impossible to get home directory");
            PathBuf::from("/")
        });

    trace!("Home directory path: {}", home_directory.display());

    let config_directory:PathBuf = home_directory.join(CONFIG_DIRECTORY_NAME);

    trace!("Config directory path: {}", config_directory.display());

    
    create_directory_if_missing(config_directory.as_path());

    trace!("Reading arguments!");
    
    let args:CliArgs = CliArgs::parse();

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
            let record_name: &String = &arg.name;
            let record_path: PathBuf = config_directory.join(record_name);

            if !is_file_missing(record_path.as_path())? {
                let mut buffer = Vec::new();
                match read_from_file(record_path.as_path(), &mut buffer) {
                    Ok(_) => {
                        match decrypt_data(buffer,String::from("Something")) {
                            Ok(plaintext) => trace!("Done: {}!", String::from_utf8(plaintext.clone()).unwrap()),
                            Err(e) =>error!("Error {}",e),
                        }
                    },
                    Err(e) => error!("Error {}",e)
                }
                
               
                
                
            }
            else {
                trace!("File {} already exists! Use update command", record_path.display());
            }
            
            
        },

        Commands::Add(arg) => {
            let record_name: &String = &arg.name;
            let record_path: PathBuf = config_directory.join(record_name);

            if is_file_missing(record_path.as_path())? {
                trace!("Add record {}; Jump to interactive mode", record_name);
                let record_data: RecordData = run_interactive()?;

                let formatted_data:String = format_content(record_data);
           
                

                match encrypt_data(formatted_data.as_bytes().to_vec(),String::from("Something")) {
                    Ok(ciphertext) => {
                        match write_to_file(record_path.as_path(), &ciphertext) {
                            Ok(_) => trace!("Successfully saved data to file!"),
                            Err(e) =>error!("Error {}",e),
                        }
                    }
                    Err(e) =>error!("Error {}",e),
                }

                
            }
            else {
                trace!("File {} already exists! Use update command", record_path.display());
            }
            
           

        },
        Commands::Update(arg) => {
            let record_name: &String = &arg.name;
            let record_path: PathBuf = config_directory.join(record_name);

            if !is_file_missing(record_path.as_path())? {
                trace!("Add record {}; Jump to interactive mode", record_name);
                let record_data: RecordData = run_interactive()?;

                let formatted_data:String = format_content(record_data);
           
                match write_to_file(record_path.as_path(), &formatted_data.as_bytes().to_vec()) {
                    Ok(_) => trace!("Successfully saved data to file!"),
                    Err(e) =>error!("Error {}",e),
                }
                //now encrypt it
            }
            else {
                trace!("File {} doesn't exists! Use add command", record_path.display());
            }
            
           
        }
        
    }

    


    Ok(())



   



  






    
}


fn run_interactive() -> io::Result<RecordData> {

    let mut input_username:String = String::new();
    let mut input_password: String = String::new();
    let mut other_info: Vec<String> = Vec::new();
    
    print!("Enter the username:");
    io::stdout().flush()?;

    io::stdin()
        .read_line(&mut input_username)?;

    let username:String = input_username.trim().to_string();

    print!("Enter the password:");
    io::stdout().flush()?;
    

    io::stdin()
        .read_line(&mut input_password)?;

    let password:String = input_password.trim().to_string();


    /* Read other infos*/
    println!("Other necessary informations (Ctrl+D on Linux/macOS, Ctrl+Z then Enter on Windows to end):");
    io::stdout().flush()?;
   

    for line in io::stdin().lock().lines() {
        let text = line?;
        other_info.push(text);

    }

    Ok(RecordData {
        username,
        password,
        other_info
    })

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

fn is_file_missing(file_path: &Path) -> io::Result<bool> {
    match fs::metadata(file_path) {
        Ok(meta) => {
            if meta.is_file() {
                Ok(false) //file exists
            }
            else {
                Ok(true) //path exists but no file!
            }
        },
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(true),//file doesn't exist
        Err(e) => Err(e), //other error
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


fn format_content(record_data: RecordData) -> String {
    let mut content:String = format!("username: {}\npassword: {}\n", record_data.username, record_data.password);

    if !record_data.other_info.is_empty() {
        content.push_str("other informations:\n");
        for info in &record_data.other_info {
            content.push_str(&format!("\t- {}\n", info));
        }
    }
    content
}

fn write_to_file(file_path: &Path, data: &Vec<u8>) -> std::io::Result<()>{
    let mut file:File = File::create(file_path)?;

   
    match file.write_all(data) {
        Ok(_) => {
            trace!("Successfully created file {}!", file_path.display());
        }
        Err(e) => {
            error!("Error on writing file: {}", e);
            return Err(e);
        }
    };
    Ok(())
}


fn read_from_file(file_path: &Path, data: &mut Vec<u8>) -> std::io::Result<()> {
    let mut file = File::open(file_path)?;
    file.read_to_end(data)?;
    Ok(())
}

//fn encrypt_with_password(data:Vec<u8>, password: &str) -> io::Result<()> {
//}
