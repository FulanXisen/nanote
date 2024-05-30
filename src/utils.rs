use std::{
    fs::{self, File},
    io::{self, stdin, BufRead, BufReader, Error, Read, Seek, SeekFrom, Write},
};

use anyhow::Result;
use rustyline::{error::ReadlineError, DefaultEditor};

pub fn read_last_terminal_command() -> Option<std::string::String> {
    let histfile = "/Users/fanyx/.local/share/fish/fish_history";
    if let Ok(file) = File::open(histfile) {
        let mut reader = BufReader::new(file);
        let mut buffer = [0; 1];
        let mut file_size = reader.seek(SeekFrom::End(0)).unwrap();
        let mut cmd = String::default();
        loop {
            let mut line = Vec::new();
            while file_size > 0 {
                file_size -= 1;
                reader.seek(SeekFrom::Start(file_size)).unwrap();
                reader.read_exact(&mut buffer).unwrap();
                if buffer[0] == b'\n' && !line.is_empty() {
                    break;
                }
                line.push(buffer[0]);
            }
            line.reverse();
            let line_string = String::from_utf8(line).unwrap();
            if line_string.starts_with("- cmd: ") {
                cmd = line_string.replace("- cmd: ", "").trim().to_string();
                cmd = cmd.split_whitespace().collect::<Vec<&str>>().join(" ");
                if !cmd.ends_with("nav add") {
                    break;
                }
            }
        }
        if cmd.is_empty() {
            return None;
        } else {
            return Some(cmd);
        }
    }
    None
}

pub fn prompt_user(prompt: &str) -> Result<String> {
    let mut rl = DefaultEditor::new()?;
    let readline = rl.readline(prompt)?;
    Ok(readline.trim().to_string())
}
