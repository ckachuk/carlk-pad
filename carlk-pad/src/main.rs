pub mod text_editor;

use std::io;
use std::env::args;
use crossterm::terminal;
use text_editor::TextEditor;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not turn off raw mode");
    }
}

fn main() -> io::Result<()> {
    let _clean_up = CleanUp;
    
    terminal::enable_raw_mode().expect("Could not turn on Raw mode");
    
    let args: Vec<String> = args().collect();

    if args.len() < 2{
        println!("Please provide file name as argument");
        std::process::exit(0);
    }
    

    if !std::path::Path::new(&args[1]).exists() {
        println!("File does not exist");
        std::process::exit(0);
    }

    let mut editor = TextEditor::init(&args[1]);
    editor.show_document();
    editor.run();

    Ok(())
}

