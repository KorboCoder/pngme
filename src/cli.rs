use clap::{Parser, Subcommand};


#[derive(Debug, Parser)]
#[command(name = "pngme")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}


#[derive(Debug, Subcommand)]
pub enum Commands{
    Encode{

        path: String,

        chunk_type: String,

        message: String,

        #[arg(default_value_t = String::from("output.png"))]
        output_file: String

    },

    Decode {

        path: String,

        chunk_type: String,

    },
    
    Remove {

        path: String,

        chunk_type: String

    },

    Print {

        path: String

    },

}
