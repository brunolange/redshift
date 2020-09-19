use std::fs;
use structopt::StructOpt;

const RSHDIR: &str = ".rsh";

#[derive(StructOpt, Debug)]
#[structopt(about = "redshift!")]
enum RedShift {
    Init { path: String },
    Status,
}

fn init(path: String) -> std::io::Result<()> {
    let dir = format!("{}/{}", path, RSHDIR);
    fs::create_dir(dir)
}

fn status() -> std::io::Result<()> {
    println!("Inside status!");
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = RedShift::from_args();
    match args {
        RedShift::Init { path } => init(path),
        RedShift::Status => status(),
    }
}
