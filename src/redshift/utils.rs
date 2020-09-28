use sha1::{Digest, Sha1};

pub fn sha1(data: &Vec<u8>) -> String {
    let mut hasher = Sha1::new();
    hasher.update(&data);
    format!("{:x}", hasher.finalize())
}

pub fn is_ignored(path: &String) -> bool {
    let mut ignored = false;
    let ignore_list = vec!["./.git", "./target"];
    for pattern in ignore_list {
        if path.contains(pattern) {
            ignored = true;
            break;
        }
    }
    ignored
}
