use std::{fs,io};
use std::fs::File;
use std::io::{Write, BufRead,Read};
use std::path::{Path, PathBuf};

pub fn write_to_file(file_path: &Path, data: &Vec<u8>) -> std::io::Result<()>{
    let mut file:File = File::create(file_path)?;
    file.write_all(data)?;
    Ok(())
}


pub fn read_from_file(file_path: &Path, data: &mut Vec<u8>) -> std::io::Result<()> {
    let mut file = File::open(file_path)?;
    file.read_to_end(data)?;
    Ok(())
}


pub fn is_file_missing(file_path: &Path) -> io::Result<bool> {
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

pub fn create_directory_if_missing(home_path:&Path){
    if is_directory_missing(home_path).unwrap() {
        fs::create_dir(home_path).unwrap();
    }


}

pub fn read_dirs(config_path: &Path) -> io::Result<Vec<PathBuf>>{
     let mut dir_entries = fs::read_dir(config_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    dir_entries.sort();
    Ok(dir_entries)
}

pub fn is_directory_missing(dir_path: &Path) -> io::Result<bool> {
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
