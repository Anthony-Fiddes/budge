pub mod cli {
    use std::fmt;

    #[derive(Debug)]
    pub struct InvalidCommandError {
        input: String,
    }

    impl fmt::Display for InvalidCommandError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "\"{}\" is not a valid command", self.input)
        }
    }

    #[derive(Debug)]
    pub enum Command {
        Add(Add),
    }

    impl Command {
        pub fn from_str(cmd: &str) -> Result<Self, InvalidCommandError> {
            match cmd.to_lowercase().as_str() {
                "add" => Ok(Command::Add(Add {})),
                _ => Err(InvalidCommandError {
                    input: cmd.to_string(),
                }),
            }
        }
    }

    #[derive(Debug)]
    pub struct Add {}
}

use std::env;
extern crate dirs;

fn main() {
    for argument in env::args() {
        println!("{argument}");
    }

    let data_dir = dirs::data_dir().expect("could not get user's data directory");
    println!("{}", data_dir.display());

    println!("{:?}", cli::Command::from_str("add"));
}
