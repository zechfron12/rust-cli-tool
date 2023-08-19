mod cmd;
mod execute;

use self::cmd::Cmd;
use self::execute::Execute;
pub struct Command {
    name: Cmd,
    args: Vec<String>,
}

impl Default for Command {
    fn default() -> Self {
        Self {
            name: Cmd::Echo,
            args: vec![String::from("Hello there")],
        }
    }
}

impl Command {
    pub fn execute(&self) -> Result<(), String> {
        self.name.execute(&self.args)
    }

    pub fn new(cmd_name: &String, args: &[String]) -> Self {
        Command {
            name: Cmd::from(cmd_name),
            args: args.to_vec(),
        }
    }
}
