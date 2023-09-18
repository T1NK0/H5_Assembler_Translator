use std::collections::HashMap;

// Definere datastrukturerne
#[derive(Debug, PartialEq)]
enum CommandType {
    ACommand, // @value
    CCommand, // dest=comp;jump
    LCommand, // (LABEL)
}

#[derive(Debug, PartialEq)]
struct Command {
    ctype: CommandType,
    symbol: Option<String>,
    dest: Option<String>,
    comp: Option<String>,
    jump: Option<String>,
    translated_line: Option<String>
}

// Define struct for the comp table
struct CompTable {
    map: HashMap<&'static str, &'static str>,
}

impl CompTable {
    fn new() -> Self {
        let map: HashMap<&str, &str> = [
            ("0", "0101010"),
            ("1", "0111111"),
            ("-1", "0111010"),
            ("D", "0001100"),
            ("A", "0110000"),
            ("M", "1110000"),
            ("!D", "0001101"),
            ("!A", "0110001"),
            ("!M", "1110001"),
            ("-D", "0001111"),
            ("-A", "0110011"),
            ("-M", "1110011"),
            ("D+1", "0011111"),
            ("A+1", "0110111"),
            ("M+1", "1110111"),
            ("D-1", "0001110"),
            ("A-1", "0110010"),
            ("M-1", "1110010"),
            ("D+A", "0000010"),
            ("D+M", "1000010"),
            ("D-A", "0010011"),
            ("D-M", "1010011"),
            ("A-D", "0000111"),
            ("M-D", "1000111"),
            ("D&A", "0000000"),
            ("D&M", "1000000"),
            ("D|A", "0010101"),
            ("D|M", "1010101"),
        ]
        .iter()
        .cloned()
        .collect();
        CompTable { map }
    }

    fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key).copied()
    }
}

// Define struct for the dest table
struct DestTable {
    map: HashMap<&'static str, &'static str>,
}

impl DestTable {
    fn new() -> Self {
        let map: HashMap<&str, &str> = [
            (null_dest, "000"),
            ("M", "001"),
            ("D", "010"),
            ("MD", "011"),
            ("A", "100"),
            ("AM", "101"),
            ("AD", "110"),
            ("AMD", "111"),
        ]
        .iter()
        .cloned()
        .collect();
        DestTable { map }
    }

    fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key).copied()
    }
}

// Define struct for the jump table
struct JumpTable {
    map: HashMap<&'static str, &'static str>,
}

impl JumpTable {
    fn new() -> Self {
        let map: HashMap<&str, &str> = [
            (null_jump, "000"),
            ("JGT", "001"),
            ("JEQ", "010"),
            ("JGE", "011"),
            ("JLT", "100"),
            ("JNE", "101"),
            ("JLE", "110"),
            ("JMP", "111"),
        ]
        .iter()
        .cloned()
        .collect();
        JumpTable { map }
    }

    fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key).copied()
    }
}

// Parserfunktion for A-kommandoer
fn parse_a_command(line: &str, current_variable: &mut i32) -> Command {
    let symbol = line[1..].to_string();
    let translated_line = format!("@{}", current_variable);
    *current_variable += 1;
    Command {
        ctype: CommandType::ACommand,
        symbol: Some(symbol),
        dest: None,
        comp: None,
        jump: None,
        translated_line: Some(translated_line),
    }
}

// Parserfunktion for C-kommandoer
// Parser function for C-commands
fn parse_c_command(line: &str, dest_table: &DestTable, comp_table: &CompTable, jump_table: &JumpTable) -> Command {
    let mut command = Command {
        ctype: CommandType::CCommand,
        symbol: Some(line.to_string()), // Store the entire statement in symbol
        dest: None,
        comp: None,
        jump: None,
        translated_line: None,
    };

    if let Some(equals_index) = line.find('=') {
        // Split by '=' to extract destination and computation
        let (dest_str, rest) = line.split_at(equals_index);
        command.dest = dest_table.get(dest_str.trim()).map(|s| s.to_string());

        // Remove the '=' character and continue processing
        let rest = &rest[1..];

        // Extract the computation and jump
        if let Some(semicolon_index) = rest.find(';') {
            let (comp_str, jump_str) = rest.split_at(semicolon_index);
            command.comp = comp_table.get(comp_str.trim()).map(|s| s.to_string());
            command.jump = jump_table.get(jump_str[1..].trim()).map(|s| s.to_string());
        } else {
            // No jump specified, process the remaining as computation
            command.comp = comp_table.get(rest.trim()).map(|s| s.to_string());
        }
    } else if let Some(semicolon_index) = line.find(';') {
        // No destination, split by ';' to extract computation and jump
        let (comp_str, jump_str) = line.split_at(semicolon_index);
        command.comp = comp_table.get(comp_str.trim()).map(|s| s.to_string());
        command.jump = jump_table.get(jump_str[1..].trim()).map(|s| s.to_string());
    } else {
        // No destination or jump specified, process the entire line as computation
        command.comp = comp_table.get(line.trim()).map(|s| s.to_string());
    }

    command
}


// Main loop for at læse filen og parse kommandoerne
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("C:/Users/mat/Documents/School/h5/nand2tetris/projects/06/add/Add.asm");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    // Our starting variable since  0-15 is r0, r1 etc. updated for each iteration.
    let mut current_variable = 16;

    // Create instances of the tables
    let comp_table = CompTable::new();
    let dest_table = DestTable::new();
    let jump_table = JumpTable::new();

    // Sets command to mutable. (Allows it to be changed.)
    let mut command: Command;
    for line in reader.lines() {
        let line = line?;
        let trimmed_line = line.trim();
    
        if trimmed_line.is_empty() || trimmed_line.starts_with("//") {
            continue;
        }
    
        let mut command: Command;
        let translated_line = format!("@{}", current_variable);
    
        if line.starts_with('@') {
            command = parse_a_command(&line, &mut current_variable);
        } else {
            command = parse_c_command(&line, &dest_table, &comp_table, &jump_table);
        }
    
        println!("{:?}", command);
    }
    
    
    Ok(())
}

// Define null values
const null_dest: &str = "";
const null_jump: &str = "";