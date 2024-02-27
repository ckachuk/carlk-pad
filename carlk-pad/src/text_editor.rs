use std::io::{self, stdout, Read, Write};
use std::fs;

use crossterm::{cursor, execute, terminal};
use crossterm::terminal::{Clear, ClearType};
use crossterm::style::{Print, Stylize};


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
    terminal_size: Coordinates
}

impl TextEditor{
    pub fn init(file_name: &str) -> Self{
        let mut doc_file = Doc{lines: vec![]};

        let file = fs::read_to_string(file_name).unwrap();
        for line in file.lines(){
            doc_file.lines.push(line.to_string());
        }

        let doc_length = file.lines().count();
        let size = terminal::size().unwrap();
        Self{
            file_name: file_name.into(),
            doc_length: doc_length,
            doc_content: doc_file,
            curr_pos: Coordinates{
                x: 1,
                y: doc_length
            },
            terminal_size: Coordinates{
                x: size.0 as usize,
                y: size.1 as usize,
            }
        }
    }

    pub fn show_document(&mut self)->io::Result<()>{
        let pos = &self.curr_pos;
        let mut stdout = stdout();
        execute!(stdout,
            Clear(ClearType::All),
            cursor::MoveTo(0,0),
        )?;
       
        println!("{}\r", "Welcome to carlk-pad".white().on_dark_cyan().bold());


        for line in 0..self.doc_length{
            println!("{}", self.doc_content.lines[line as usize]);
        }
        let _ = stdout.flush();


        execute!(stdout,
            cursor::MoveTo(0, (self.terminal_size.y - 3) as u16),
        )?;

        println!("{}{}{}{}\r", "File length: ".white().on_dark_cyan().bold()
            , self.doc_length.to_string().white().on_dark_cyan().bold()
            ," You are in the file ".white().on_dark_cyan().bold(), self.file_name.to_string().white().on_dark_cyan().bold());

        Ok(())

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