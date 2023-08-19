use std::{
    fs::{read_to_string, write},
    io::Error,
};
pub trait Execute {
    fn execute(&self, args: &[String]) -> Result<(), String>;
}

pub fn echo(s: &str) -> Result<(), String> {
    if s.is_empty() {
        return Err(String::from("Empty string provided"));
    }
    println!("{}", s);
    Ok(())
}

pub fn cat(dest_path: &str, from_path: &str) -> Result<(), Error> {
    let contents = read_to_string(from_path)?;
    write(dest_path, contents)
}
