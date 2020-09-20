mod redshift;
mod utils;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "redshift!")]
enum RedShift {
    Init { path: String },
    Status,
}

fn main() -> std::io::Result<()> {
    let args = RedShift::from_args();
    utils::hello();
    match args {
        RedShift::Init { path } => redshift::init(path),
        RedShift::Status => redshift::status(),
    }
}
