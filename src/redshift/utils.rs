use sha1::{Digest, Sha1};

pub fn sha1(data: &Vec<u8>) -> String {
    let mut hasher = Sha1::new();
    hasher.update(&data);
    format!("{:x}", hasher.finalize())
}
