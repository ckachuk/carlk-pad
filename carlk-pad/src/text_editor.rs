use std::io::{self, stdout, Read, Write};
use std::fs;

use crossterm::cursor;
use crossterm::terminal::{Clear, ClearType};
use crossterm::QueueableCommand;


struct Doc{
    lines: Vec<String>,
}

struct Coordinates{
    pub x: usize,
    pub y: usize,
}

pub struct TextEditor{
    file_name: String,
    doc_length: usize,
    doc_content: Doc,
    curr_pos: Coordinates,
}

impl TextEditor{
    pub fn init(file_name: &str) -> Self{
        let mut doc_file = Doc{lines: vec![]};

        let file = fs::read_to_string(file_name).unwrap();
        for line in file.lines(){
            doc_file.lines.push(line.to_string());
        }

        let doc_length = file.lines().count();

        Self{
            file_name: file_name.into(),
            doc_length: doc_length,
            doc_content: doc_file,
            curr_pos: Coordinates{
                x: 1,
                y: doc_length
            }
        }
    }

    pub fn show_document(&mut self){
        let pos = &self.curr_pos;
        let mut stdout = stdout();
        let _ = stdout.queue(Clear(ClearType::All));
        let _ = stdout.queue(cursor::MoveTo(1,1));
        println!("{}", self.file_name);

        for line in 0..self.doc_length{
            println!("{}", self.doc_content.lines[line as usize]);
        }
        let _ = stdout.flush();
    }

    pub fn run(&mut self){
        let mut buf = [0; 1];
        while io::stdin().read(&mut buf).expect("Failed to read the line") == 1 && buf != [17]{
            let character = buf[0] as char;
            if character.is_control(){
                println!("{}", character as u8);
            }
            else{
                println!("{}", character);
            }
        }
    }
}