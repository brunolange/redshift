use std::fs;

mod utils;

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
    let oid = utils::sha1(&data);

    fs::write(format!(".rsh/objects/{}", oid), data)
}

pub fn cat_file(oid: String) -> std::io::Result<()> {
    let contents = fs::read_to_string(format!(".rsh/objects/{}", oid))?;
    print!("{}", contents);
    println!();
    Ok(())
}

mod tests {
    #[allow(unused)]
    use super::*;

    #[test]
    fn test_status() {
        let result = status();
        assert!(result.is_ok());
    }
}
