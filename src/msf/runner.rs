use std::process::Command;

#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    pub msf_version: (bool, String),
    pub msfvenom_version: (bool, String),
    pub ruby_version: (bool, String),
    pub db_status: (bool, String),
    pub module_counts: Vec<(String, usize)>,
}

pub fn check_msf_installed() -> (bool, String) {
    match Command::new("msfconsole").arg("--version").output() {
        Ok(output) => {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .next()
                    .unwrap_or("unknown")
                    .to_string();
                (true, version)
            } else {
                (false, "error".to_string())
            }
        }
        Err(_) => (false, "not found".to_string()),
    }
}

pub fn check_msfvenom_installed() -> (bool, String) {
    match Command::new("msfvenom").arg("--version").output() {
        Ok(output) => {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .next()
                    .unwrap_or("unknown")
                    .to_string();
                (true, version)
            } else {
                (false, "error".to_string())
            }
        }
        Err(_) => (false, "not found".to_string()),
    }
}

pub fn check_ruby_version() -> (bool, String) {
    match Command::new("ruby").arg("--version").output() {
        Ok(output) => {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .next()
                    .unwrap_or("unknown")
                    .trim()
                    .to_string();
                (true, version)
            } else {
                (false, "error".to_string())
            }
        }
        Err(_) => (false, "not found".to_string()),
    }
}

pub fn check_db_status() -> (bool, String) {
    match Command::new("msfconsole")
        .args(["-q", "-x", "db_status; exit"])
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let line = stdout.lines().find(|l| l.contains("connected")).unwrap_or("");
            if !line.is_empty() {
                (true, line.trim().to_string())
            } else {
                let msg = stdout.lines().find(|l| l.contains("db_status")).unwrap_or("not available");
                (false, msg.trim().to_string())
            }
        }
        Err(_) => (false, "could not query".to_string()),
    }
}

pub fn count_modules() -> Vec<(String, usize)> {
    let output = match Command::new("msfconsole")
        .args(["-q", "-x", "show -a; exit"])
        .output()
    {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).to_string(),
        _ => {
            let categories = ["exploit", "auxiliary", "payload", "post", "encoder", "nop", "evasion"];
            return categories.iter().map(|c| (c.to_string(), 0)).collect();
        }
    };

    let categories = ["exploit", "auxiliary", "payload", "post", "encoder", "nop", "evasion"];
    let mut counts: Vec<(String, usize)> = categories.iter().map(|c| (c.to_string(), 0)).collect();

    for line in output.lines() {
        for (i, cat) in categories.iter().enumerate() {
            if line.trim_start().starts_with(&format!("{cat}/")) {
                counts[i].1 += 1;
                break;
            }
        }
    }
    counts
}

pub fn quick_check() -> HealthCheckResult {
    HealthCheckResult {
        msf_version: check_msf_installed(),
        msfvenom_version: check_msfvenom_installed(),
        ruby_version: check_ruby_version(),
        db_status: (false, "press [h] to check".to_string()),
        module_counts: vec![
            ("exploit".to_string(), 0),
            ("auxiliary".to_string(), 0),
            ("payload".to_string(), 0),
            ("post".to_string(), 0),
            ("encoder".to_string(), 0),
            ("nop".to_string(), 0),
            ("evasion".to_string(), 0),
        ],
    }
}

pub fn run_health_check() -> HealthCheckResult {
    HealthCheckResult {
        msf_version: check_msf_installed(),
        msfvenom_version: check_msfvenom_installed(),
        ruby_version: check_ruby_version(),
        db_status: check_db_status(),
        module_counts: count_modules(),
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
