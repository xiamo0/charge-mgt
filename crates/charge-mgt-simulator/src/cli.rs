use std::process;

#[derive(Debug, Clone)]
pub struct CliConfig {
    pub gateway_url: String,
    pub charge_point_id: String,
    pub no_color: bool,
}

pub fn parse_args() -> CliConfig {
    let args: Vec<String> = std::env::args().collect();
    let mut gateway_url: Option<String> = None;
    let mut charge_point_id: Option<String> = None;
    let mut no_color = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-g" | "--gateway" => {
                i += 1;
                gateway_url = args.get(i).cloned();
            }
            "--id" => {
                i += 1;
                charge_point_id = args.get(i).cloned();
            }
            "--no-color" => {
                no_color = true;
            }
            "-h" | "--help" => {
                print_help();
                process::exit(0);
            }
            unknown => {
                eprintln!("unknown argument: {unknown}");
                eprintln!();
                print_help();
                process::exit(2);
            }
        }
        i += 1;
    }

    let gateway_url = match gateway_url {
        Some(u) => u,
        None => {
            eprintln!("error: --gateway is required");
            eprintln!();
            print_help();
            process::exit(2);
        }
    };

    CliConfig {
        gateway_url,
        charge_point_id: charge_point_id.unwrap_or_else(|| "SIM-001".to_string()),
        no_color,
    }
}

fn print_help() {
    eprintln!(
        "charge_mgt_simulator — OCPP 1.6 interactive REPL simulator

USAGE:
    charge_mgt_simulator --gateway <URL> [--id <CHARGE_POINT_ID>] [--no-color]

OPTIONS:
    -g, --gateway <URL>         Gateway WebSocket base URL (required)
                                Example: ws://127.0.0.1:9000
        --id <ID>               Charge point id appended to URL path (default: SIM-001)
        --no-color              Disable ANSI colors in output
    -h, --help                  Print this help

EXAMPLE:
    charge_mgt_simulator -g ws://127.0.0.1:9000 --id CP-001"
    );
}
