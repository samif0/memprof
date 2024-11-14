use std::error::Error;
use nix::sys::ptrace;
use nix::unistd::{Pid, fork, ForkResult, geteuid};
use std::process::Command;

pub struct StackTracer {
    pid: Pid,
}

impl StackTracer {
    pub fn new(pid: i32) -> Self {
        Self { pid: Pid::from_raw(pid) }
    }

    pub fn capture(&self) -> Result<Vec<String>, Box<dyn Error>> {
        if !Self::has_ptrace_access()? {
            return Err("Root privileges required for stack tracing".into());
        }
        ptrace::attach(self.pid)?;
        let bt = backtrace::Backtrace::new();
        let frames: Vec<String> = bt.frames()
            .iter()
            .map(|f| format!("{:?}", f))
            .collect();
        ptrace::detach(self.pid, None)?;
        Ok(frames)
    }

    fn has_ptrace_access() -> Result<bool, Box<dyn Error>> {
        Ok(geteuid().is_root())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_stack_capture() {
        if !StackTracer::has_ptrace_access().unwrap() {
            println!("Test requires root privileges, skipping...");
            return;
        }

        let mut child = Command::new("sleep")
            .arg("10")
            .spawn()
            .expect("Failed to spawn test process");

        let pid = child.id() as i32;
        thread::sleep(Duration::from_millis(100));
        
        let tracer = StackTracer::new(pid);
        let trace = tracer.capture().expect("Failed to capture stack trace");
        assert!(!trace.is_empty());

        child.kill().expect("Failed to kill test process");
    }
}