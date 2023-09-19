pub struct FileHandler;
use std::fs::{File, self};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

impl FileHandler {
    pub fn ReadFromFile(path:&str) -> io::Result<String> {
        fs::read_to_string(path)
    }

    pub fn SaveToHack(translated_text:Vec<u16>, file_path:&str) {
        let mut path = PathBuf::from(file_path);
        path.set_extension("hack");

        let mut hack_bytes = String::new(); 

        let mut new_file: File = File::create(&path).expect("Can't make file.");
        
        for line in translated_text  {
            hack_bytes.push_str(&format!("{:016b}\n", line))
        }
        
        new_file.write_all(hack_bytes.trim_end().as_bytes()).expect("Can't write to file.");
    }
}