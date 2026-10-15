use std::any::type_name;
// use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::str::FromStr;
use std::string::String;

#[derive(Debug)]
struct MetaData {
    version: u8,
    framerate: u8,
    height: u16,
    width: u16,
    frame_count: u16,
}

impl MetaData {
    fn default() -> Self {
        Self {
            version: 0,
            framerate: 0,
            frame_count: 0,
            height: 0,
            width: 0,
        }
    }
}

#[derive(Debug, Default)]
struct Frame {
    id: u16,
    content: Vec<String>,
}

impl Frame {
    fn new() -> Self {
        Self {
            id: 0,
            content: Vec::new(),
        }
    }

    fn default() -> Self {
        Self {
            id: 0,
            content: Vec::new(),
        }
    }

    fn reuse(&mut self, id: u16) {
        self.id = id;
        self.content.clear();
    }

    fn push(&mut self, line: String) {
        self.content.push(line);
    }

    fn is_empty(&mut self) -> bool {
        self.content.is_empty()
    }
}

#[derive(Debug)]
struct Tac {
    metadata: MetaData,
    frames: Vec<Frame>,
}

impl Tac {
    // new tac from file
    fn new(f: File) -> Self {
        let reader = BufReader::new(f);

        let mut md = MetaData::default();
        let mut frames: Vec<Frame> = Vec::new();
        let mut frame: Frame = Frame::new(); // frame initialized to 0
        // let mut frame_end: bool = false;

        // tac parsing
        for line in reader.lines() {
            match line {
                Ok(line_content) => {
                    let mut trimmed_line = line_content.trim();
                    //handle metadata
                    if trimmed_line.starts_with("[") && trimmed_line.ends_with("]") {
                        trimmed_line = &(trimmed_line[1..trimmed_line.len() - 1]);

                        let tmp = trimmed_line.split(": ").collect::<Vec<&str>>();
                        let key = tmp[0];
                        let val: usize = usize::from_str(tmp[1].trim()).unwrap();

                        match key {
                            "version" => md.version = val as u8,
                            "width" => md.width = val as u16,
                            "height" => md.height = val as u16,
                            "frameRate" => md.framerate = val as u8,
                            "frameCount" => md.frame_count = val as u16,
                            "frame" => {
                                // on every new frame, assuming the frames are read in order
                                if !(val as u16 == 0) {
                                    // push last frame into frames if not the first
                                    // and re-initialize
                                    frames.push(std::mem::take(&mut frame));
                                    // frame.id will be re-initialized to 0 so you have to
                                    // re use last frame
                                    frame.reuse(val as u16);
                                }
                            }
                            _ => eprintln!("invalid metadata"),
                        }
                    } else {
                        //handle frames
                        frame.push(line_content.to_string());
                    }
                }
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }

        if !frame.is_empty() {
            frames.push(std::mem::take(&mut frame));
        }

        Self {
            metadata: md,
            frames: frames,
        }
    }

    fn is_empty(&self) -> bool {
        if self.metadata.frame_count == 0 {
            true
        } else {
            false
        }
    }

    fn get_metadata(&self) -> &MetaData {
        &self.metadata
    }
}

#[derive(Debug)]
struct TacPlayer {
    animation: Tac,
    _loop: bool,
}

fn main() -> io::Result<()> {
    let input_file_path = "./assets/pulsar.tac";
    let file = File::open(input_file_path)?;
    let pulsar = Tac::new(file);
    println!("{:#?}", pulsar);
    println!("{:#?}", pulsar.get_metadata());

    Ok(())
}
