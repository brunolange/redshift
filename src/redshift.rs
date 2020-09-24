use sha1::{Digest, Sha1};
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

pub fn hash_object(path: String) -> std::io::Result<()> {
    let data = fs::read(path)?;

    // create a Sha1 object
    let mut hasher = Sha1::new();

    // process input message
    hasher.update(data);

    // acquire hash digest in the form of GenericArray,
    // which in this case is equivalent to [u8; 20]
    let result = hasher.finalize();

    println!("result = {:?}", result);

    Ok(())
}
