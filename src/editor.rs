use std::{fs::File, io::{self, BufReader, Read}, path::PathBuf};

#[derive(Debug, thiserror::Error)]
enum EditorError {
    #[error("no file chosen")]
    NoFileChosen,

    //lime IO Errors
    #[error("failed to open '{}': {src}", .path.display())]
    Open { path: PathBuf, src: io::Error},
    #[error("failed to read '{}': {src}", .path.display())]
    Read { path: PathBuf, src: io::Error },
    #[error("failed to write '{}': {src}", .path.display())]
    Write { path: PathBuf, src: io::Error }
}

pub struct Editor {
    openend_files: Vec<PathBuf>,
    chosen_file: Option<PathBuf>,
    folder: Option<PathBuf>,
    content: String
}

impl Editor {
    pub fn new() -> Self {
        Self {
            openend_files: Vec::new(),
            chosen_file: None,
            folder: None,
            content: String::new()
        }
    }

    pub fn new_file(&mut self) {
        self.chosen_file = None;
        self.content.clear();
    }

    pub fn open_file(&mut self, path: impl Into<PathBuf>) {
        self.openend_files.push(path.into());
    }

    pub fn chose_file(&mut self, chosen: impl Into<PathBuf>) {
        self.chosen_file = Some(chosen.into());
        match self.insert_content() {
            Ok(_) => (),
            Err(e) => eprintln!("{e}")
        };
    }

    fn insert_content(&mut self) -> Result<(), EditorError> {
        if let Some(path) = &mut self.chosen_file {
            let file = File::open(&path).map_err(|src| EditorError::Open { path: path.to_path_buf(), src })?;        
            let mut reader = BufReader::new(file);

            reader.read_to_string(&mut self.content).map_err(|src| EditorError::Read {path: path.to_path_buf(), src } )?;
        }
        Err(EditorError::NoFileChosen)
    }

    pub fn update(&mut self) {

    }

    pub fn render(&self) {
        
    }
}