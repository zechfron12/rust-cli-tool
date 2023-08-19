pub enum Cmd {
    Echo,
    Cat,
    Ls,
    Find,
    Grep,
    None,
}

pub trait Execute {
    fn execute(&self, arg: &str) -> Result<(), String>;
}

impl Execute for Cmd {
    fn execute(&self, arg: &str) -> Result<(), String> {
        match self {
            Cmd::Echo => {
                println!("{}", arg);
                Ok(())
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
