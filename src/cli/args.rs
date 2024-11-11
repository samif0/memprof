use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub pid: i32,
}

impl Args {
    pub fn get_args() -> Self {
        Self::parse()
    }
}
