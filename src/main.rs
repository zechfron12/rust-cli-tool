mod command;

use command::Command;
use std::env::args;

fn main() -> Result<(), String> {
    let args: Vec<String> = args().collect();
    let name = args.get(1);

    match name {
        Some(cmd_name) => {
            let (_, cmd_args) = args.split_at(2);
            Command::new(cmd_name, cmd_args).execute()
        }
        None => Err("No command inserted".into()),
    }
}
