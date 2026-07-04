use std::process::Command;

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

pub fn check_msfvenom_installed() -> bool {
    Command::new("msfvenom")
        .arg("--version")
        .output()
        .is_ok_and(|o| o.status.success())
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
