use clap::{Args, Parser, Subcommand};

/// Password saver
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CliArgs {
        
    #[command(subcommand)]
    pub command: Commands,
    
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all records
    List,
    
    ///Show a specific record
    Show(ShowRecordArg),

    ///Add a record
    Add(AddRecordArg),

    ///Update a record
    Update(AddRecordArg),

    ///Generate password
    Generate(GenerateRecordArg),
}


#[derive(Args)]
pub struct ShowRecordArg {
    #[arg(name = "name", help = " Name of the record.")]
    pub name: String,
    #[arg(name = "password" , help = "Password use to encrypt/decrypt.")]
    pub password: String,
}


#[derive(Args)]
pub struct AddRecordArg {

    #[arg(name = "name", help = "Name of the record.")]
    pub name: String,

    #[arg(name = "password" , help = "Password use to encrypt/decrypt.")]
    pub password: String,
}


#[derive(Args)]
pub struct GenerateRecordArg {
    #[arg(name ="length", help = "Password length.")]
    pub length: u16,
}
