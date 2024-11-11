use std::process::Command;
use std::str::FromStr;
use crate::cli::Args;


#[derive(Debug, PartialEq)]
pub struct Process {
    pub uid: u32,
    pub pid: u32,
    pub ppid: u32,
    pub flags: u32,
    pub cpu_pct: u32,
    pub pri: u32,
    pub nice: u32,
    pub sz: u32, //KB
    pub rss: u32, //KB
    pub wchan: String,
    pub state: String,
    pub paddr: u32,
    pub tty: String,
    pub time: String,
    pub cmd: String,
}

impl FromStr for Process {
    type Err = Box<dyn std::error::Error>; //TODO: Make new error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();


        if parts.len() < 15 {
            return Err("Not enough fields to parse Process".into());
        }

        Ok(Process {
            uid: parts[0].parse()?,
            pid: parts[1].parse()?,
            ppid: parts[2].parse()?,
            flags: parts[3].parse()?,
            cpu_pct: parts[4].parse()?,
            pri: parts[5].parse()?,
            nice: parts[6].parse()?,
            sz: parts[7].parse()?,
            rss: parts[8].parse()?,
            wchan: parts[9].to_string(),
            state: parts[10].to_string(),
            paddr: parts[11].parse()?,
            tty: parts[12].to_string(),
            time: parts[13].to_string(),
            cmd: parts[14].to_string(),
        })
    }
}



impl Process {
    pub fn new(args: Args) -> Self {
        let output = Command::new("ps")
        .args(["-p", &args.pid.to_string(), "-l"])
        .output()
        .expect("Failed to execute ps");

        //TODO: check if pid exists and fail gracefully

        let out_str = String::from_utf8(output.stdout).unwrap();
        match out_str.trim().lines().nth(1) {
            Some(val) => {
                match Process::from_str(val) {
                    Ok(ret) => return ret,
                    Err(err) => {
                        panic!("{err}");
                    }
                }
            },
            None => {
                panic!("Could not parse process");
            }
        }
    }
}