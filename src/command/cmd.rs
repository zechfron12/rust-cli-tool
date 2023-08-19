pub use super::execute::Execute;
use super::execute::{cat, echo, find, grep, ls};
pub enum Cmd {
    Echo,
    Cat,
    Ls,
    Find,
    Grep,
    None,
}

impl Execute for Cmd {
    fn execute(&self, args: &[String]) -> Result<(), String> {
        match self {
            Cmd::Echo => echo(args),
            Cmd::Cat => cat(args),
            Cmd::Ls => ls(args),
            Cmd::Find => find(args),
            Cmd::Grep => grep(args),
            Cmd::None => Err(String::from("Command not found")),
        }
    }
}

impl From<&String> for Cmd {
    fn from(value: &String) -> Self {
        let name: &str = value.as_str();
        match name {
            "echo" => Cmd::Echo,
            "cat" => Cmd::Cat,
            "ls" => Cmd::Ls,
            "find" => Cmd::Find,
            "grep" => Cmd::Grep,
            _ => Cmd::None,
        }
    }
}
