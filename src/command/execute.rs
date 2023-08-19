use std::{
    fs::{read_to_string, write},
    io::Error,
};
pub trait Execute {
    fn execute(&self, args: &[String]) -> Result<(), String>;
}

pub fn echo(args: &[String]) -> Result<(), String> {
    if let Some(s) = args.get(0) {
        if s.is_empty() {
            return Err(String::from("Empty string provided"));
        }
        println!("{}", s);
        Ok(())
    } else {
        Err(String::from("Could not get the parameter"))
    }
}

pub fn cat(args: &[String]) -> Result<(), String> {
    let dest_path = args.get(0);
    let from_path = args.get(1);

    if dest_path.is_none() || from_path.is_none() {
        return Err("2 paths are needed".into());
    }

    let dest_path = dest_path.unwrap();
    let from_path = from_path.unwrap();
    let contents_from =
        read_to_string(from_path).map_err(|_| String::from("Could not read from file"))?;
    let contents_dest = read_to_string(dest_path)
        .map_err(|_| String::from("Could not read from file"))?
        + contents_from.as_str();
    write(dest_path, contents_dest).map_err(|_| String::from("Could not write in file"))?;
    Ok(())
}
