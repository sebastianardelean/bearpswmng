mod cli;
mod bearcrypto;
mod file;
use log::{trace,error};
use log4rs;

use std::io;
use home;
use std::path::PathBuf;
use std::io::{Write, BufRead};
use clap::Parser;
use cli::{CliArgs, Commands};
use rand::Rng;


use bearcrypto::{encrypt, decrypt};
use file::{write_to_file,read_from_file,is_file_missing, create_directory_if_missing,read_dirs};
use base64::prelude::*;



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
                        match decrypt(buffer, arg.password.clone()) {
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
           
                

                match encrypt(formatted_data.as_bytes().to_vec(),arg.password.clone()) {
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
            
           
        },

        Commands::Generate(arg) => {
            let password_length: u16 = arg.length;
            let password:String = generate_random_string(password_length.into());

            trace!("The password is: {}", password);
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



fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"1234567890qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";
    const SPECIAL_CHARSET: &[u8] = b" !\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
    let mut rng = rand::rng();

    let rand_chars: String = (0..length)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();


    let b64_pass = BASE64_STANDARD.encode(rand_chars.as_bytes());
    let mut final_pass:String = String::with_capacity(length);

    for _ in 0..length {
        let pick_b64 = rng.random_bool(0.5); // 50/50 chance
        if pick_b64 {
            let idx = rng.random_range(0..b64_pass.len());
            final_pass.push(b64_pass.as_bytes()[idx] as char);
        } else {
            let idx = rng.random_range(0..SPECIAL_CHARSET.len());
            final_pass.push(SPECIAL_CHARSET[idx] as char);
        }
    }

    return final_pass;
}
