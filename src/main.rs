mod command;

use command::Command;
use std::env::args;

fn main() -> Result<(), String> {
    let args: Vec<String> = args().collect();
    let cmd_name = args.get(1).expect("Could not get fist param");
    let cmd_args = args.get(2).expect("Could not get second param");
    Command::new(cmd_name, cmd_args).execute()
}
