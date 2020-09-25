use sha1::{Digest, Sha1};
use std::fs;

const RSHDIR: &str = ".rsh";

pub fn init(path: String) -> std::io::Result<()> {
    fs::create_dir(format!("{}/{}", path, RSHDIR))?;
    fs::create_dir(format!("{}/{}/objects", path, RSHDIR))?;
    Ok(())
}

pub fn status() -> std::io::Result<()> {
    println!("Inside status!");
    Ok(())
}

pub fn hash_object(path: String) -> std::io::Result<()> {
    let data = fs::read(path)?;
    let mut hasher = Sha1::new();
    hasher.update(&data);
    let oid = format!("{:x}", hasher.finalize());

    fs::write(format!(".rsh/objects/{}", oid), data)
}

mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_status() {
        let result = status();
        assert!(result.is_ok());
    }
}
