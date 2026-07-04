use std::process::Command;

#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    pub msf_version: (bool, String),
    pub msfvenom_version: (bool, String),
    pub ruby_version: (bool, String),
    pub db_status: (bool, String),
}

fn run_cmd(cmd: &str, args: &[&str]) -> (bool, String) {
    match Command::new(cmd).args(args).output() {
        Ok(output) => {
            let mut combined = String::new();
            if !output.stdout.is_empty() {
                combined.push_str(&String::from_utf8_lossy(&output.stdout));
            }
            if !output.stderr.is_empty() {
                if !combined.is_empty() {
                    combined.push('\n');
                }
                combined.push_str(&String::from_utf8_lossy(&output.stderr));
            }
            (output.status.success(), combined)
        }
        Err(e) => (false, format!("error: {e}")),
    }
}

fn first_line(text: &str) -> String {
    text.lines()
        .find(|l| !l.is_empty())
        .unwrap_or("unknown")
        .trim()
        .to_string()
}

pub fn check_msf_installed() -> (bool, String) {
    let (ok, output) = run_cmd("msfconsole", &["--version"]);
    if ok {
        let version = first_line(&output);
        if version.is_empty() || version == "unknown" {
            (false, "not detected".to_string())
        } else {
            (true, version)
        }
    } else {
        let version = first_line(&output);
        if version.is_empty() || version == "unknown" {
            (false, "not found".to_string())
        } else {
            (true, version)
        }
    }
}

pub fn check_msfvenom_installed() -> (bool, String) {
    let (ok, output) = run_cmd("msfvenom", &["--version"]);
    if ok {
        let version = first_line(&output);
        if version.is_empty() || version == "unknown" {
            (false, "not detected".to_string())
        } else {
            (true, version)
        }
    } else {
        let version = first_line(&output);
        if version.is_empty() || version == "unknown" {
            (false, "not found".to_string())
        } else {
            (true, version)
        }
    }
}

pub fn check_ruby_version() -> (bool, String) {
    let (ok, output) = run_cmd("ruby", &["--version"]);
    if ok {
        let version = first_line(&output);
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
