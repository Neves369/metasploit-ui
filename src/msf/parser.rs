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
