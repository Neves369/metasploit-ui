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

use crate::msf::runner::run_command;

pub fn update_msf() -> Result<String, String> {
    run_command("msfupdate", &[])
}
