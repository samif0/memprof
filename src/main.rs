use clap::Parser;
use std::process::Command;
//use mach; TODO: Use syscalls instead of ps call

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    pid: i32,
}


fn main() {
    let args = Args::parse();
    
    let output = Command::new("ps")
        .args(["-p", &args.pid.to_string(), "-o", "rss"])
        .output()
        .expect("Failed to execute ps");

    if let Ok(output_str) = String::from_utf8(output.stdout) {
        if let Some(memory) = output_str.lines().nth(1) {
            println!("Memory usage: {} KB", memory.trim());
        }
    }
}
