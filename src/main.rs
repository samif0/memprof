mod cli;
mod memory;

use cli::Args;
use memory::Process;

//use mach; TODO: Use syscalls instead of ps call

fn main() {
    let args = Args::get_args();
    let proc = Process::new(args);
    
    println!("{:?}", proc);
} 
