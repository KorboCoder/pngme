use cli::Cli;
use clap::Parser;
use png::Png;

use std::str::FromStr;

use crate::{chunk_type::ChunkType, chunk::Chunk};

mod cli;
mod chunk;
mod chunk_type;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T,Error>;


fn load_png(path: String) -> std::result::Result<Png, ()> {

    match std::fs::read(path) {
        Ok(bytes) => Png::try_from(bytes.as_slice()),
        Err(err) => panic!("Unable to load png file: {}", err) 
    }
}

fn save_png(png: Png, path: String) -> std::result::Result<(), std::io::Error> {
    std::fs::write(path, png.as_bytes())
}

fn main() -> Result<()>{

    let args = Cli::parse();

    match args.command {

        cli::Commands::Encode { path, chunk_type, message, output_file } => {

            let mut png = load_png(path).expect("Unable to read png.");
            let chunk_type = ChunkType::from_str(&chunk_type).expect("Invalid chunk_type");
            let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());
            png.append_chunk(chunk);
            save_png(png, output_file).expect("Error saving output file");
        },

        cli::Commands::Decode { path, chunk_type } => {
            let png = load_png(path).expect("Unable to read png.");
            let chunk = png.chunk_by_type(&chunk_type);
            if let Some(chunk) = chunk {
                let message = chunk.data_as_string().expect("Error encoding data");
                println!("{}", message);
            }
            else{
                println!("Nothing to decode");
            }
        },
            
        cli::Commands::Remove { path, chunk_type } => {
            let mut png = load_png(path.clone()).expect("Unable to read png.");
            png.remove_chunk(&chunk_type).expect("Error removing chunk.");
            save_png(png, path).expect("Error saving output file.");
            println!("Removed encoded message")
        },

        cli::Commands::Print { path } => {
            match load_png(path) {
                Ok(png) => println!("{}", png.to_string()),
                Err(_) => panic!("Unable to read png.")
            }
        },

    }

    Ok(())

}
