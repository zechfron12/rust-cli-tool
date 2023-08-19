use std::env::args;

enum Cmd {
    Echo,
    Cat,
    Ls,
    Find,
    Grep,
    None,
}

trait Execute {
    fn execute(&self, arg: &String) -> Result<(), String>;
}

impl Execute for Cmd {
    fn execute(&self, arg: &String) -> Result<(), String> {
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

struct Command {
    name: Cmd,
    arg: String,
}

impl Default for Command {
    fn default() -> Self {
        Self {
            name: Cmd::Echo,
            arg: String::from("Hello there"),
        }
    }
}

impl Command {
    fn execute(&self) -> Result<(), String> {
        return self.name.execute(&self.arg);
    }

    fn new(cmd_name: &String, arg: &String) -> Self {
        Command {
            name: Cmd::from(cmd_name),
            arg: arg.clone(),
        }
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = args().collect();
    let cmd_name = args.get(1).expect("Could not get fist param");
    let cmd_args = args.get(2).expect("Could not get second param");
    Command::new(cmd_name, cmd_args).execute()
}
