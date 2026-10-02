use std::env;
/// Global application configuration.
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Port the HTTP server listens on.
    pub port: u16,
    /// Bind address, e.g.
    pub host: String,
}
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            host: "0.0.0.0".to_string(),
        }
    }
}
impl AppConfig {
    /// Build config with priority:
    ///   1. command-line args (--port, --host)
    ///   2. environment variables (APP_PORT, APP_HOST)
    ///   3. defaults
    pub fn load() -> Self {
        let mut cfg = Self::default();
        // environment variables
        if let Ok(port) = env::var("APP_PORT") {
            match port.parse::<u16>() {
                Ok(p) => cfg.port = p,
                Err(_) => eprintln!("invalid APP_PORT={:?}, ignoring", port),
            }
        }
        if let Ok(host) = env::var("APP_HOST") {
            cfg.host = host;
        }
        // command-line args (highest priority)
        let args: Vec<String> = env::args().skip(1).collect();
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--port" | "-p" => {
                    if let Some(v) = args.get(i + 1) {
                        match v.parse::<u16>() {
                            Ok(p) => cfg.port = p,
                            Err(_) => eprintln!("invalid --port={:?}, ignoring", v),
                        }
                        i += 2;
                    } else {
                        eprintln!("--port requires a value");
                        i += 1;
                    }
                }
                "--host" | "-h" => {
                    if let Some(v) = args.get(i + 1) {
                        cfg.host = v.clone();
                        i += 2;
                    } else {
                        eprintln!("--host requires a value");
                        i += 1;
                    }
                }
                "--help" => {
                    print_help();
                    std::process::exit(0);
                }
                other => {
                    eprintln!("unknown argument: {}", other);
                    print_help();
                    std::process::exit(1);
                }
            }
        }
        cfg
    }
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
fn print_help() {
    println!(
        r#"Usage: app [OPTIONS]
Options:
  -p, --port <PORT>   Port to listen on   (env: APP_PORT, default: 3000)
  -h, --host <HOST>   Host to bind        (env: APP_HOST, default: 127.0.0.1)
      --help          Show this help
"#
    );
}
