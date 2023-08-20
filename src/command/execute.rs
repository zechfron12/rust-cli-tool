use std::{
    fs::{read_dir, read_to_string, write},
    path::Path,
};

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

fn display_dir_files_rec<P>(path: P) -> Result<(), String>
where
    P: AsRef<Path>,
{
    let paths = read_dir(path).map_err(|_| String::from("Could not read directory"))?;

    for path in paths {
        let entry = path.map_err(|_| String::from("Could not get dir entry"))?;

        println!("{}", entry.path().display());
        if let Ok(metadata) = entry.metadata() {
            if metadata.is_dir() {
                display_dir_files_rec(entry.path())?;
            }
        }
        // print!("{} ", path.unwrap().path().display())
    }

    Ok(())
}

pub fn find(args: &[String]) -> Result<(), String> {
    let mut name = String::from("./");
    let binding = String::new();
    let arg = args.get(0).unwrap_or(&binding);
    name.push_str(arg.as_str());

    let paths = read_dir("./").map_err(|_| String::from("Could not read directory"))?;

    for path in paths {
        let entry = path.map_err(|_| String::from("Could not get dir entry"))?;
        if entry.path().starts_with(&name) {
            println!("{} ", entry.path().display());

            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    display_dir_files_rec(entry.path())?;
                }
            }
        }
    }

    Ok(())
}

pub fn grep(args: &[String]) -> Result<(), String> {
    todo!()
}
