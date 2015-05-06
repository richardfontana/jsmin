use std::error::Error;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    
    let arg = env::args().nth(1);
  
    match arg {
        Some(s) => {
            let path = Path::new(&s);
            let display = path.display();
            let mut file = match File::open(&path) {
                Err(why) => panic!("couldn't open {}: {}", display,
                                   Error::description(&why)),
                Ok(file) => file,
            };
            let mut buf = String::new();
            match file.read_to_string(&mut buf) {
                Err(why) => panic!("couldn't read {}: {}", display,
                                   Error::description(&why)),
                Ok(_)    => print!("{} contains\n{}", display, buf),
            }
        },
        None    => println!("Usage: jsmin <FILENAME>"),
    }
}

