use self::cmd::{Cmd, Execute};

mod cmd;

pub struct Command {
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
    pub fn execute(&self) -> Result<(), String> {
        self.name.execute(&self.arg)
    }

    pub fn new(cmd_name: &String, arg: &str) -> Self {
        Command {
            name: Cmd::from(cmd_name),
            arg: String::from(arg),
        }
    }
}
