use clap::Parser;
use colored::Colorize;
use sysproxy::Sysproxy;

mod args;

fn main() {
    let args = args::Args::parse();
    let sysproxy = Sysproxy::get_system_proxy().unwrap();

    match args {
        args::Args {
            launch: false,
            on: false,
            off: false,
            status: false,
            dashboard: false,
            kill: false,
        } => {
            println!("Run without any args.");
        }
        _ => {
            if args.launch {
                println!("Use launch arg.");
            }

            if args.on {
                println!("Use on arg.");
            }

            if args.off {
                println!("Use off arg.");
            }

            if args.status {
                println!("Use status arg.");
                println!("{}", status(&sysproxy));
            }

            if args.dashboard {
                println!("Use dashboard arg.");
            }

            if args.kill {
                println!("Use kill arg.");
            }
        }
    }
}

fn status(sysproxy: &Sysproxy) -> String {
    let enable = if sysproxy.enable {
        &"on".green()
    } else {
        &"off".red()
    };
    let host = &sysproxy.host;
    let port = sysproxy.port;
    let bypass = &sysproxy.bypass;
    format!(
        r#"
>>> Docker Clash Status <<<
System proxy: {enable}
Clash Docker:
Proxy [host:port]: {host}:{port}
Bypass: {bypass}
"#
    )
}
