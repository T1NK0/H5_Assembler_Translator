use std::{env, path};
use std::io::{self, BufRead};
mod file_handler;
use crate::file_handler::FileHandler;
mod parse_handler;
use crate::parse_handler::ParseHandler;

// Main loop for at læse filen og parse kommandoerne
fn main() -> io::Result<()> {
    let arguments: Vec<String> = env::args().collect();
    let file_path = match arguments.get(1){
        Some(path)if path.ends_with(".asm") => path,
        _ => {
            panic!("No file");
        }
    };
    
    let lines:String = FileHandler::ReadFromFile(&file_path)?.into();

    let mut parse_handler = ParseHandler::new();
    let result = parse_handler.parse_to_binary(lines);

    FileHandler::SaveToHack(result, &file_path);

    Ok(())
}