mod command;

use command::Command;
use std::env::args;

fn main() -> Result<(), String> {
    let args: Vec<String> = args().collect();
    let (name_slice, cmd_args) = args.split_at(2);
    let name = &name_slice.get(1);

    if name.is_none() {
        return Err("Could not read the command".into());
    };

    let name = name.unwrap();
    Command::new(name, cmd_args.to_vec()).execute()
}
