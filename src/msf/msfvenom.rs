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
