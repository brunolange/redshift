use std::fs;

const RSHDIR: &str = ".rsh";

pub fn init(path: String) -> std::io::Result<()> {
    let dir = format!("{}/{}", path, RSHDIR);
    match fs::create_dir(&dir) {
        Err(error) => {
            println!("Got the following error: {}", error);
            Err(error)
        }
        Ok(()) => {
            println!("Initialized empty Redshift repository in {}", dir);
            Ok(())
        }
    }
}

pub fn status() -> std::io::Result<()> {
    println!("Inside status!");
    Ok(())
}
