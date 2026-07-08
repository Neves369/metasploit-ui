use std::io::Read;
use std::sync::mpsc;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    pub msf_version: (bool, String),
    pub msfvenom_version: (bool, String),
    pub ruby_version: (bool, String),
    pub db_status: (bool, String),
}

fn run_cmd(cmd: &str, args: &[&str]) -> (bool, String) {
    run_cmd_with_timeout(cmd, args, DEFAULT_TIMEOUT)
}

fn run_cmd_with_timeout(cmd: &str, args: &[&str], timeout: Duration) -> (bool, String) {
    let mut child = match Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => return (false, format!("error: {e}")),
    };

    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();

    let (stdout_tx, stdout_rx) = mpsc::channel();
    let (stderr_tx, stderr_rx) = mpsc::channel();

    std::thread::spawn(move || {
        let mut buffer = String::new();
        let _ = stdout.read_to_string(&mut buffer);
        let _ = stdout_tx.send(buffer);
    });

    std::thread::spawn(move || {
        let mut buffer = String::new();
        let _ = stderr.read_to_string(&mut buffer);
        let _ = stderr_tx.send(buffer);
    });

    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let stdout = stdout_rx.recv().unwrap_or_default();
                let stderr = stderr_rx.recv().unwrap_or_default();
                let output = combine_output(stdout, stderr);
                return (status.success(), output);
            }
            Ok(None) => {
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    let stdout = stdout_rx.recv().unwrap_or_default();
                    let stderr = stderr_rx.recv().unwrap_or_default();
                    let output = combine_output(stdout, stderr);
                    return (
                        false,
                        format!(
                            "timeout: {cmd} did not respond within {timeout:?}\n{output}"
                        ),
                    );
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(e) => {
                return (false, format!("error: {e}"));
            }
        }
    }
}

fn combine_output(stdout: String, stderr: String) -> String {
    if stdout.is_empty() {
        stderr
    } else if stderr.is_empty() {
        stdout
    } else {
        format!("{stdout}\n{stderr}")
    }
}

fn first_line(text: &str) -> String {
    text.lines()
        .find(|l| !l.is_empty())
        .unwrap_or("unknown")
        .trim()
        .to_string()
}

fn check_cli_version(cmd: &str, args: &[&str]) -> (bool, String) {
    let (ok, output) = run_cmd(cmd, args);
    let version = first_line(&output);

    if ok {
        if version.is_empty() || version == "unknown" {
            (false, "not detected".to_string())
        } else {
            (true, version)
        }
    } else {
        if version.is_empty() || version == "unknown" {
            (false, "not found".to_string())
        } else {
            (false, version)
        }
    }
}

pub fn check_msf_installed() -> (bool, String) {
    check_cli_version("msfconsole", &["--version"])
}

pub fn check_msfvenom_installed() -> (bool, String) {
    check_cli_version("msfvenom", &["--version"])
}

pub fn check_ruby_version() -> (bool, String) {
    let (ok, output) = run_cmd("ruby", &["--version"]);
    let version = first_line(&output);
    if ok {
        if version.is_empty() || version == "unknown" {
            (false, "not detected".to_string())
        } else {
            (true, version)
        }
    } else {
        (false, "not found".to_string())
    }
}

pub fn check_db_status() -> (bool, String) {
    let (_ok, output) = run_cmd("msfconsole", &["-q", "-x", "db_status; exit"]);
    let output = output.trim();
    let output_lower = output.to_lowercase();
    if output_lower.contains("connected") {
        let msg = output
            .lines()
            .find(|l| l.to_lowercase().contains("database"))
            .unwrap_or("connected");
        (true, msg.trim().to_string())
    } else if output_lower.contains("postgres") {
        (true, "connected to postgresql".to_string())
    } else {
        let msg = output
            .lines()
            .find(|l| l.to_lowercase().contains("database"))
            .unwrap_or("not connected");
        (false, msg.trim().to_string())
    }
}

pub fn quick_check() -> HealthCheckResult {
    HealthCheckResult {
        msf_version: check_msf_installed(),
        msfvenom_version: check_msfvenom_installed(),
        ruby_version: check_ruby_version(),
        db_status: (false, "press [h] to check".to_string()),
    }
}

pub fn run_health_check() -> HealthCheckResult {
    HealthCheckResult {
        msf_version: check_msf_installed(),
        msfvenom_version: check_msfvenom_installed(),
        ruby_version: check_ruby_version(),
        db_status: check_db_status(),
    }
}

pub fn run_command(cmd: &str, args: &[&str]) -> Result<String, String> {
    Command::new(cmd)
        .args(args)
        .output()
        .map(|o| {
            let stdout = String::from_utf8_lossy(&o.stdout).to_string();
            let stderr = String::from_utf8_lossy(&o.stderr).to_string();
            if o.status.success() {
                stdout
            } else {
                format!("{stdout}\n{stderr}")
            }
        })
        .map_err(|e| format!("Failed to execute {cmd}: {e}"))
}

pub fn run_msf_command(msf_args: &[&str]) -> Result<String, String> {
    run_command("msfconsole", &["-q", "-x", &msf_args.join(";")])
}
