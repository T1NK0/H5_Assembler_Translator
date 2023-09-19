use std::collections::HashMap;

// Definere datastrukturerne
enum CommandType {
    ACommand, // @value
    CCommand, // dest=comp;jump
    LCommand, // (LABEL)
}
// Definere commands
struct Command {
    ctype: CommandType,
    symbol: Option<String>,
    dest: Option<String>,
    comp: Option<String>,
    jump: Option<String>
}

pub struct ParseHandler {
    // TODO: Add later
    command_type: CommandType,
    variable_counter: u16,
    label_lineshift: usize,
    symbol_map: HashMap<String, u16>,
}

impl ParseHandler {
    pub fn new() -> ParseHandler {
        let map: HashMap<String, u16> = [
            ("SP".to_string(), 0),
            ("LCL".to_string(), 1),
            ("ARG".to_string(), 2),
            ("THIS".to_string(), 3),
            ("THAT".to_string(), 4),
            ("R0".to_string(), 0),
            ("R1".to_string(), 1),
            ("R2".to_string(), 2),
            ("R3".to_string(), 3),
            ("R4".to_string(), 4),
            ("R5".to_string(), 5),
            ("R6".to_string(), 6),
            ("R7".to_string(), 7),
            ("R8".to_string(), 8),
            ("R9".to_string(), 9),
            ("R10".to_string(), 10),
            ("R11".to_string(), 11),
            ("R12".to_string(), 12),
            ("R13".to_string(), 13),
            ("R14".to_string(), 14),
            ("R15".to_string(), 15),
            ("SCREEN".to_string(), 16384),
            ("KBD".to_string(), 24576),
        ].into();

        ParseHandler{
            command_type: CommandType::ACommand,
            variable_counter: 16,
            label_lineshift: 0,
            symbol_map: map
        }
    }

    pub fn parse_to_binary(&mut self, file_lines:String) -> Vec<u16> {
        let mut cleaned_file = Self::file_cleaner(&file_lines);
        self.find_all_labels(&cleaned_file);
        cleaned_file = Self::remove_all_labels(&cleaned_file);
        self.translate(&cleaned_file)
    }

    fn file_cleaner(file_lines:&str) -> String {
        file_lines
            .lines()
            .filter_map(|x| x.split("//").next())
            .map(|y| y.replace(" ", ""))
            .filter(|z| !z.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn find_all_labels(&mut self, cleaned_file:&str) {
        for (i, line) in cleaned_file.lines().enumerate() {
            if line.starts_with("(") {
                let symbol = line.trim_matches(|c| c == '(' || c == ')').to_string();
                if !self.symbol_map.contains_key(&symbol) {
                    self.symbol_map.insert(symbol, (i-self.label_lineshift).try_into().unwrap());
                    self.label_lineshift += 1;
                }
            }
        }
    }

    fn remove_all_labels (file_lines:&str) -> String{
        file_lines
            .lines()
            .filter(|x| !x.starts_with("("))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn translate(&mut self, cleaned_file:&str) -> Vec<u16> {
        cleaned_file
            .lines()
            .map(|x| self.command_definer(x))
            .collect::<Vec<u16>>()
    }

    fn command_definer(&mut self, file_line:&str) -> u16 {
        match file_line.starts_with('@') {
            true => {self.translate_a_command(file_line)},
            false => {self.translate_c_command(file_line)}
        }
    }

    fn translate_a_command(&mut self, file_line:&str) -> u16 {
        let a_command = &file_line[1..];
        dbg!(&a_command);

        if let Ok(value) = a_command.parse::<u16>() {
            return value;
        }

        match self.symbol_map.get(a_command) {
            Some(values) => *values,
            None => {
                let address = self.variable_counter;
                dbg!(&address);
                self.symbol_map.insert(a_command.to_string(), address);
                self.variable_counter += 1;
                address
            },
        }

    }

    fn translate_c_command(&self, file_line:&str) -> u16 {
        dbg!(&file_line);
        let prepped_line = match file_line.contains('=') {
            true => file_line.into(),
            false => {format!("null={}", file_line)}
        };
        let c_command:Vec<&str> = prepped_line.split(&['=', ';']).collect();

        let destination_bits = Self::dest_in_bits(c_command.get(0).unwrap_or(&""));
        let computation_bits = Self::comp_in_bits(c_command.get(1).unwrap_or(&""));
        let jump_bits = Self::jump_in_bits(c_command.get(2).unwrap_or(&""));
        let a_bit = if c_command.get(1).unwrap_or(&"").contains("M") {"1"} else {"0"};

        let completed_c_command = format!("111{}{}{}{}", a_bit, computation_bits, destination_bits, jump_bits);

        // Set's debug. (Needs pointer or you can't borrow the variable later.)
        dbg!(&completed_c_command);
        
        u16::from_str_radix(&completed_c_command, 2).unwrap()
    }

    fn dest_in_bits(dest:&str) -> &str {
        match dest {
            "M" => "001",
            "D" => "010",
            "MD" => "011",
            "A" => "100",
            "AM" => "101",
            "AD" => "110",
            "AMD" => "111",
            _ => "000"
        }
    }

    fn comp_in_bits(comp:&str) -> &str {
        match comp {
            "0"                => "101010",
            "1"                => "111111",
            "-1"               => "111010",
            "D"                => "001100",
            "A"    | "M"       => "110000",
            "!D"               => "001101",
            "!A"   | "!M"      => "110001",
            "-D"               => "001111",
            "-A"   | "-M"      => "110011",
            "D+1"              => "011111",
            "A+1"  | "M+1"     => "110111",
            "D-1"              => "001110",
            "A-1"  | "M-1"     => "110010",
            "D+A"  | "D+M"     => "000010",
            "D-A"  | "D-M"     => "010011",
            "A-D"  | "M-D"     => "000111",
            "D&A"  | "D&M"     => "000000",
            "D|A"  | "D|M"     => "010101",
            _ => ""
        }
    }

    fn jump_in_bits(jump:&str) -> &str {
        match jump {
            "JGT" => "001",
            "JEQ" => "010",
            "JGE" => "011",
            "JLT" => "100",
            "JNE" => "101",
            "JLE" => "110",
            "JMP" => "111",
            _ => "000"
        }
    }
}