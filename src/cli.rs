use clap::{Args, Parser, Subcommand};

/// Simple program to greet a person
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
}


#[derive(Args)]
pub struct ShowRecordArg {
    #[arg(name = "name", help = " Name of the record.")]
    pub name: String,
}


#[derive(Args)]
pub struct AddRecordArg {

    #[arg(name = "name", help = "Name of the record.")]
    pub name: String,
}

