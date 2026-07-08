#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub name: String,
    pub rank: String,
    pub description: String,
}

pub fn parse_module_list(output: &str) -> Vec<ModuleInfo> {
    let mut modules = Vec::new();
    let mut in_table = false;

    for line in output.lines() {
        if line.contains("=====") {
            in_table = true;
            continue;
        }

        if !in_table {
            continue;
        }

        if line.trim().is_empty() || line.contains("msf6") || line.contains("[-]") {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let name = parts[0].to_string();
            let rank = parts.get(1).unwrap_or(&"unknown").to_string();
            let description = parts[2..].join(" ");

            if !name.starts_with("=") && !name.is_empty() {
                modules.push(ModuleInfo {
                    name,
                    rank,
                    description,
                });
            }
        }
    }

    modules
}

pub fn parse_search_results(output: &str) -> Vec<ModuleInfo> {
    let mut results = Vec::new();
    let mut in_results = false;

    for line in output.lines() {
        if line.contains("Matching Modules") || line.contains("====") {
            in_results = true;
            continue;
        }

        if !in_results {
            continue;
        }

        if line.trim().is_empty() || line.contains("msf6") || line.contains("[-]") {
            continue;
        }

        let parts: Vec<&str> = line.splitn(3, |c: char| c == ' ' || c == '\t').collect();
        let filtered: Vec<&str> = parts.iter().filter(|s| !s.is_empty()).copied().collect();

        if filtered.len() >= 2 {
            let name = filtered[0].to_string();
            let rank = if filtered.len() > 1 { filtered[1].to_string() } else { "unknown".into() };
            let description = if filtered.len() > 2 {
                filtered[2..].join(" ")
            } else {
                String::new()
            };

            results.push(ModuleInfo {
                name,
                rank,
                description,
            });
        }
    }

    results
}

pub fn extract_version(output: &str) -> String {
    output
        .lines()
        .next()
        .unwrap_or("unknown")
        .trim()
        .to_string()
}

pub fn parse_session_list(output: &str) -> Vec<(u32, String, String)> {
    let mut sessions = Vec::new();
    for line in output.lines() {
        if line.trim().is_empty() || !line.trim().starts_with(|c: char| c.is_ascii_digit()) {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            if let Ok(id) = parts[0].parse::<u32>() {
                sessions.push((
                    id,
                    parts.get(1).unwrap_or(&"").to_string(),
                    parts.get(2).unwrap_or(&"").to_string(),
                ));
            }
        }
    }
    sessions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_module_list_extracts_entries() {
        let output = "Header\n=====\nexploit/windows/smb/ms17_010_eternalblue excellent MS17-010 EternalBlue SMB Remote Windows Kernel Pool Corruption\nauxiliary/scanner/ssh/ssh_login normal SSH login scanner\n";
        let modules = parse_module_list(output);

        assert_eq!(modules.len(), 2);
        assert_eq!(modules[0].name, "exploit/windows/smb/ms17_010_eternalblue");
        assert_eq!(modules[0].rank, "excellent");
        assert!(modules[0]
            .description
            .starts_with("MS17-010 EternalBlue SMB Remote"));

        assert_eq!(modules[1].name, "auxiliary/scanner/ssh/ssh_login");
        assert_eq!(modules[1].rank, "normal");
        assert!(modules[1].description.contains("SSH login scanner"));
    }

    #[test]
    fn parse_search_results_extracts_results() {
        let output = "Matching Modules\n====\nexploit/windows/smb/ms17_010_eternalblue excellent MS17-010 EternalBlue SMB Remote Windows Kernel Pool Corruption\nauxiliary/scanner/ssh/ssh_login normal SSH login scanner\n";
        let results = parse_search_results(output);

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].name, "exploit/windows/smb/ms17_010_eternalblue");
        assert_eq!(results[0].rank, "excellent");
        assert!(results[0]
            .description
            .contains("EternalBlue SMB Remote"));
        assert_eq!(results[1].name, "auxiliary/scanner/ssh/ssh_login");
    }

    #[test]
    fn parse_session_list_returns_sessions() {
        let output = "\nid   type  information\n---- ------ --------------------------\n1    meterpreter x86/linux    127.0.0.1:4444 -> 127.0.0.1:1234\n2    shell      x86/linux    127.0.0.1:4444 -> 127.0.0.1:1235\n";
        let sessions = parse_session_list(output);

        assert_eq!(sessions.len(), 2);
        assert_eq!(sessions[0].0, 1);
        assert_eq!(sessions[0].1, "meterpreter");
        assert_eq!(sessions[0].2, "x86/linux");
    }
}
