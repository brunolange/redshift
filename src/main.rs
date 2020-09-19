use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "redshift!")]
enum RedShift {
    Init { path: std::path::PathBuf },
    Status,
}

fn init(path: std::path::PathBuf) -> Option<()> {
    println!("Inside init!");
    println!("path = {:?}", path);
    Some(())
}

fn status() -> Option<()> {
    println!("Inside status!");
    Some(())
}

fn main() {
    let args = RedShift::from_args();
    let result = match args {
        RedShift::Init { path } => init(path),
        RedShift::Status => status(),
    };
    println!("result = {:?}", result);
}
