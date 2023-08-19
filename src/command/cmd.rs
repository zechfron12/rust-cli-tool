use super::execute::echo;
pub use super::execute::Execute;
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
            Cmd::Echo => {
                if let Some(s) = args.get(0) {
                    echo(s)
                } else {
                    Err(String::from("Could not get the parameter"))
                }
            }
            Cmd::Cat => todo!(),
            Cmd::Ls => todo!(),
            Cmd::Find => todo!(),
            Cmd::Grep => todo!(),
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
