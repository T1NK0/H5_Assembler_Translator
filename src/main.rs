use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
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
    jump: Option<String>
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
    *current_variable += 1;
    Command {
        ctype: CommandType::ACommand,
        symbol: Some(symbol),
        dest: None,
        comp: None,
        jump: None,
    }
}

// Parser function for C-commands
fn parse_c_command(line: &str, dest_table: &DestTable, comp_table: &CompTable, jump_table: &JumpTable) -> Command {
    let mut command = Command {
        ctype: CommandType::CCommand,
        symbol: Some(line.to_string()), // Store the entire statement in symbol
        dest: None,
        comp: None,
        jump: None,
    };


    // If the line contains a '=' execute following code
    if let Some(equals_index) = line.find('=') {
        // Set the dest_str and the rest variables thats mutable, since it's &str, if split the values from both sides of the equal to the respected variables.
        let (dest_str, rest) = line.split_at(equals_index);
        // Sets the command variable dest. to the value from our destination table, that matches the key, from our line.
        command.dest = dest_table.get(dest_str.trim()).map(|s| s.to_string());

        // Remove the first char which now is the '=', and then leave the rest of the get stored in the rest variable.
        let rest = &rest[1..];

        // Check if there is a jump value in the query, by searching for semicolon in the rest variable. 
        if let Some(semicolon_index) = rest.find(';') {
            // Set the comp and jump variables to the split values at the semicolon index.
            let (comp_str, jump_str) = rest.split_at(semicolon_index);
            // Set the commands comp value by looking for the key in the comp table and set the value of the key.
            command.comp = comp_table.get(comp_str.trim()).map(|s| s.to_string());
            // Set the jump value by looking for the key in the jump table, and removing the semicolon from the string we search with by indexing after 1.
            command.jump = jump_table.get(jump_str[1..].trim()).map(|s| s.to_string());
        } else {
            // No jump specified, process the remaining as computation
            command.comp = comp_table.get(rest.trim()).map(|s| s.to_string());
        }
        // Else if it just contains a semicolon and no destination
    } else if let Some(semicolon_index) = line.find(';') {
        // Set the computation and jump strings to the values on each side of the split.
        let (comp_str, jump_str) = line.split_at(semicolon_index);
        // Set the computation of command to the value from the key.
        command.comp = comp_table.get(comp_str.trim()).map(|s| s.to_string());
        // Set the jump value of the command, to the key after trimming off the middle char, to the value.
        command.jump = jump_table.get(jump_str[1..].trim()).map(|s| s.to_string());
    } else {
        // No destination or jump specified, process the entire line as computation
        command.comp = comp_table.get(line.trim()).map(|s| s.to_string());
    }

    command
}

// Main loop for at læse filen og parse kommandoerne
fn main() -> io::Result<()> {
    // The path of the file we're translating.
    let path = Path::new("C:/Users/mat/Documents/School/h5/nand2tetris/projects/06/add/Add.asm");
    // Open the file, in the path variable.
    let file = File::open(&path)?;
    // Read the file, using the "BufReader"
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
        // Removes the whitespace from the lines, and save the trimmed line to the new variable "trimmed_line"
        let trimmed_line = line.trim();
        // If the trimmed line is empty, or is a commented line, skip the line.
        if trimmed_line.is_empty() || trimmed_line.starts_with("//") {
            continue;
        }
        
        // Creates a new Command line for each iteration.
        let mut command: Command;
    
        // IF the line starts with a '@' to indicate an "ACommand"
        if line.starts_with('@') {
            // pass along the line we are on, and the start value for the variables (16, since 0 - 15 is R0, R1. etc.)
            command = parse_a_command(&line, &mut current_variable);
            // set the result to the command values.
            let result: Option<i16> = command
                    .symbol
                    .as_ref()
                    .map(|s| s.parse::<i16>().ok())
                    .flatten();

            match result {
                // Adds the extra 0's needed if need be and print the binary code.
                Some(value) => {
                    let binary_str = format!("{:016b}", value);
                    println!("{}", binary_str);
                }
                None => {
                    println!("Failed to parse integer.");
                }
            }
            // Checks for labels.
        } else if line.starts_with("(") {
            
        
        } else {
            // Else it's a CCommand

            // Set command, equal to the variables we assign values to in the parse_c_command function
            command = parse_c_command(&line, &dest_table, &comp_table, &jump_table);
            // Set the result with the formatted string using the macro "format".
            let result = format!(
                //This will add 3 1's to the front of the string, and the brackets will be the next 3 variables (Comp, dest, and jump.)
                "111{}{}{}",
                command.comp.unwrap_or_default(),
                command.dest.unwrap_or_default(),
                // Set the variable, or if null, add 000.
                command.jump.unwrap_or_else(|| "000".to_string())
            );
            // Writes the result to the console.
            println!("{}", result);
        }    
    }   
    
    Ok(())
}