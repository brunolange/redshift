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

pub fn hash_object(path: String, kind: &str) -> std::io::Result<()> {
    let data = fs::read(path)?;

    let mut obj = vec![];
    obj.extend(kind.as_bytes());
    obj.push(0); // the null byte
    obj.extend(&data);

    let oid = utils::sha1(&obj);
    fs::write(format!(".rsh/objects/{}", oid), obj)?;
    println!("Saved object {:?}", oid);
    Ok(())
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
