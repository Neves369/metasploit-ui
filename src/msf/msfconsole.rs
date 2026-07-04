use crate::msf::runner::run_command;
use crate::msf::runner::run_msf_command;
use crate::msf::parser;

pub fn search_modules(query: &str) -> Result<Vec<parser::ModuleInfo>, String> {
    let output = run_msf_command(&[&format!("search {query}"), "exit"])?;
    Ok(parser::parse_module_list(&output))
}

pub fn get_module_info(module: &str) -> Result<String, String> {
    run_msf_command(&[&format!("info {module}"), "exit"])
}

pub fn show_modules(category: &str) -> Result<Vec<parser::ModuleInfo>, String> {
    let output = run_msf_command(&[&format!("show {category}"), "exit"])?;
    Ok(parser::parse_module_list(&output))
}

pub fn check_db_status() -> Result<String, String> {
    run_msf_command(&["db_status", "exit"])
}

pub fn init_db() -> Result<String, String> {
    run_command("msfdb", &["init"])
}

pub fn update_msf() -> Result<String, String> {
    run_command("msfupdate", &[])
}

pub fn list_sessions() -> Result<String, String> {
    run_msf_command(&["sessions -l", "exit"])
}

pub fn kill_session(id: u32) -> Result<String, String> {
    run_msf_command(&[&format!("sessions -k {id}"), "exit"])
}

pub fn upgrade_session(id: u32) -> Result<String, String> {
    run_msf_command(&[&format!("sessions -u {id}"), "exit"])
}

pub fn run_resource_script(path: &str) -> Result<String, String> {
    run_command("msfconsole", &["-q", "-r", path])
}
