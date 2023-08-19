use std::fs::{read_dir, read_to_string, write};
pub trait Execute {
    fn execute(&self, args: &[String]) -> Result<(), String>;
}

pub fn echo(args: &[String]) -> Result<(), String> {
    if let Some(s) = args.get(0) {
        println!("{}", s);
        Ok(())
    } else {
        Err(String::from("Could not get the parameter"))
    }
}

pub fn cat(args: &[String]) -> Result<(), String> {
    let dest_path = args.get(0).ok_or(String::from("2 paths are needed"))?;
    let from_path = args.get(1).ok_or(String::from("2 paths are needed"))?;

    let contents_from =
        read_to_string(from_path).map_err(|_| String::from("Could not read from file"))?;

    let contents_dest = read_to_string(dest_path)
        .map_err(|_| String::from("Could not read from file"))?
        + contents_from.as_str();

    write(dest_path, contents_dest).map_err(|_| String::from("Could not write in file"))?;
    Ok(())
}

pub fn ls(args: &[String]) -> Result<(), String> {
    let binding = String::from("./");
    let path = args.get(0).unwrap_or(&binding);
    let paths = read_dir(path).map_err(|_| String::from("Could not read directory"))?;

    for path in paths {
        print!("{} ", path.unwrap().path().display())
    }

    Ok(())
}

pub fn find(args: &[String]) -> Result<(), String> {
    todo!()
}

pub fn grep(args: &[String]) -> Result<(), String> {
    todo!()
}
