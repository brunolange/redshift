use sha1::{Digest, Sha1};

pub fn sha1(data: &Vec<u8>) -> String {
    let mut hasher = Sha1::new();
    hasher.update(&data);
    format!("{:x}", hasher.finalize())
}

pub fn is_ignored(path: &String) -> bool {
    let ignore_list = vec!["./.git", "./target"];
    ignore_list.iter().any(|&pattern| path.contains(pattern))
}
