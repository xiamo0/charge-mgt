use charge_mgt_simulator::cli::parse_args;
use charge_mgt_simulator::heartbeat::HeartbeatScheduler;
use charge_mgt_simulator::ocpp::client::{IncomingEvent, OcppClient};
use charge_mgt_simulator::repl::parser::{self, ReplCommand, COMMAND_TABLE};
use charge_mgt_simulator::repl::writer::{self, WriterOptions};
use charge_mgt_simulator::session::log::Log;

use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use serde_json::json;
use tokio::task::spawn_blocking;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .init();

    if let Err(e) = run().await {
        eprintln!("fatal: {e}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = parse_args();
    let ws_url = format!(
        "{}/{}",
        cli.gateway_url.trim_end_matches('/'),
        cli.charge_point_id
    );

    let writer_opts = WriterOptions::new(cli.no_color);
    if cli.no_color {
        colored::control::set_override(false);
    }

    println!("🔌 Connecting to {ws_url} ...");

    let (client, event_rx) = match OcppClient::connect(&ws_url).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} connection failed: {e}", writer_opts.paint("✗", |s| s.red()));
            return Err(e.into());
        }
    };

    println!(
        "{} Connected as {}",
        writer_opts.paint("✓", |s| s.green()),
        cli.charge_point_id
    );
    println!();
    println!("Type 'help' for commands, 'quit' or Ctrl+D to exit.");

    let mut rl = DefaultEditor::new()?;
    let mut event_rx = event_rx;
    let mut log: Log = Log::new(1000);
    let mut heartbeat: Option<HeartbeatScheduler> = None;

    let mut rl_join = spawn_blocking(move || {
        let res = rl.readline("> ");
        (rl, res)
    });

    loop {
        tokio::select! {
            biased;
            ev = event_rx.recv() => {
                match ev {
                    Some(incoming) => {
                        log.record_recv(raw_event_text(&incoming));
                        println!("{}", writer::format_incoming(&incoming, &writer_opts));
                    }
                    None => {
                        println!("{}", writer_opts.paint("🔌 event channel closed", |s| s.dimmed()));
                        break;
                    }
                }
                // rl_join 仍 pending，等下一轮继续 await
                continue;
            }
            input = &mut rl_join => {
                let (mut rl_returned, line_res) = input?;
                match line_res {
                    Ok(text) => {
                        let _ = rl_returned.add_history_entry(&text);
                        if let Some(cmd) = parser::parse(&text) {
                            if matches!(cmd, ReplCommand::Quit) {
                                drop(rl_returned);
                                break;
                            }
                            handle_command(
                                cmd,
                                &text,
                                &client,
                                &writer_opts,
                                &mut log,
                                &mut heartbeat,
                            ).await;
                        }
                        rl = rl_returned;
                    }
                    Err(ReadlineError::Interrupted) => {
                        // Ctrl+C: 清当前行，留在 REPL
                        rl = rl_returned;
                    }
                    Err(ReadlineError::Eof) => {
                        println!("\n🔌 EOF, exiting...");
                        drop(rl_returned);
                        break;
                    }
                    Err(e) => {
                        eprintln!("readline error: {e}");
                        drop(rl_returned);
                        break;
                    }
                }
                // 用新 rl 重新调度下一次 readline
                rl_join = spawn_blocking(move || {
                    let res = rl.readline("> ");
                    (rl, res)
                });
            }
        }
    }

    if let Some(h) = heartbeat.take() {
        h.stop();
    }
    client.shutdown();
    // 短暂等待 task 退出
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    println!("{}", writer_opts.paint("🔌 Disconnected", |s| s.dimmed()));
    Ok(())
}

fn raw_event_text(ev: &IncomingEvent) -> String {
    use charge_mgt_simulator::ocpp::client::IncomingMessage;
    match ev {
        IncomingEvent::Message(msg) => match msg {
            IncomingMessage::CallResult { uid, payload, .. } => {
                serde_json::to_string(&json!([3, uid, payload])).unwrap_or_default()
            }
            IncomingMessage::CallError { uid, code, description, details, .. } => {
                serde_json::to_string(&json!([4, uid, code, description, details]))
                    .unwrap_or_default()
            }
            IncomingMessage::ServerCall { uid, action, payload } => {
                serde_json::to_string(&json!([2, uid, action, payload]))
                    .unwrap_or_default()
            }
        },
        IncomingEvent::BadEnvelope { raw, .. } => raw.clone(),
        IncomingEvent::ConnectionClosed { reason } => format!("__closed__: {reason}"),
    }
}

async fn handle_command(
    cmd: ReplCommand,
    _original_text: &str,
    client: &OcppClient,
    opts: &WriterOptions,
    log: &mut Log,
    heartbeat: &mut Option<HeartbeatScheduler>,
) {
    match cmd {
        ReplCommand::Help => print_help(),
        ReplCommand::Quit => {}
        ReplCommand::Status => print_status(client, heartbeat).await,
        ReplCommand::Clear => print!("\x1b[2J\x1b[1;1H"),
        ReplCommand::History => print_history(log),
        ReplCommand::Last => print_last_recv(log),
        ReplCommand::LastSent => print_last_sent(log),
        ReplCommand::Replay { index } => replay(client, opts, log, index).await,
        ReplCommand::HeartbeatOn { interval_secs } => {
            if let Some(existing) = heartbeat.take() {
                existing.stop();
            }
            let sched = HeartbeatScheduler::start(client.clone(), interval_secs);
            println!(
                "{} Heartbeat task started (interval={}s)",
                opts.paint("✓", |s| s.green()),
                interval_secs
            );
            *heartbeat = Some(sched);
        }
        ReplCommand::HeartbeatOff => {
            if let Some(sched) = heartbeat.take() {
                let count = sched.sent_count();
                sched.stop();
                println!(
                    "{} Heartbeat task stopped ({} sent)",
                    opts.paint("✓", |s| s.green()),
                    count
                );
            } else {
                println!("No heartbeat task running.");
            }
        }
        ReplCommand::SendRaw { envelope_text } => {
            let _sent_idx = log.record_sent(envelope_text.clone());
            match client.send_raw(&envelope_text).await {
                Ok(maybe_uid) => {
                    println!("{}", writer::format_sent(&envelope_text, maybe_uid.as_deref(), opts));
                }
                Err(e) => {
                    println!("{} send failed: {e}", opts.paint("✗", |s| s.red()));
                }
            }
        }
        ReplCommand::SendCall { action, payload, uid } => {
            match client.send_call(&action, payload.clone(), uid).await {
                Ok(used_uid) => {
                    let envelope = json!([2, &used_uid, &action, &payload]);
                    let text = serde_json::to_string(&envelope).unwrap_or_default();
                    let _ = log.record_sent(text.clone());
                    println!("{}", writer::format_sent(&text, Some(&used_uid), opts));
                }
                Err(e) => {
                    println!("{} send failed: {e}", opts.paint("✗", |s| s.red()));
                }
            }
        }
        ReplCommand::Respond { uid, payload } => {
            match client.respond(&uid, payload.clone()).await {
                Ok(()) => {
                    let envelope = json!([3, &uid, &payload]);
                    let text = serde_json::to_string(&envelope).unwrap_or_default();
                    let _ = log.record_sent(text.clone());
                    println!("{}", writer::format_sent(&text, Some(&uid), opts));
                }
                Err(e) => {
                    println!("{} respond failed: {e}", opts.paint("✗", |s| s.red()));
                }
            }
        }
        ReplCommand::SendError { uid, code, description } => {
            match client.send_error(&uid, &code, &description).await {
                Ok(()) => {
                    let envelope = json!([4, &uid, &code, &description, json!({})]);
                    let text = serde_json::to_string(&envelope).unwrap_or_default();
                    let _ = log.record_sent(text.clone());
                    println!("{}", writer::format_sent(&text, Some(&uid), opts));
                }
                Err(e) => {
                    println!("{} error send failed: {e}", opts.paint("✗", |s| s.red()));
                }
            }
        }
        ReplCommand::Unknown { first_token } => print_unknown(&first_token, opts),
    }
}

async fn print_status(client: &OcppClient, heartbeat: &Option<HeartbeatScheduler>) {
    let pending_user = client.pending_user_count().await;
    let mut server_calls = client.pending_server_list().await;
    server_calls.sort_by_key(|(_, info)| info.at);
    println!("connection:     active");
    println!("msgs_sent:      {}", client.msgs_sent());
    println!("msgs_recv:      {}", client.msgs_recv());
    println!("pending_user:   {pending_user}");
    println!("pending_server: {}", server_calls.len());
    for (uid, info) in server_calls {
        let action = info.action.as_deref().unwrap_or("?");
        let elapsed = info.at.elapsed();
        println!("  • {action} uid={uid} elapsed={elapsed:?}");
    }
    match heartbeat {
        Some(h) => {
            let sent = h.sent_count();
            let last = h.last_sent();
            println!(
                "heartbeat_task: running (interval={}s, sent={sent}, last={:?})",
                h.interval().as_secs(),
                last.map(|i| i.elapsed())
            );
        }
        None => println!("heartbeat_task: stopped"),
    }
}

fn print_history(log: &Log) {
    let entries: Vec<_> = log.sent_history().collect();
    if entries.is_empty() {
        println!("(no history yet)");
        return;
    }
    for e in entries {
        println!("{:>4}. {}", e.index, e.text);
    }
}

fn print_last_recv(log: &Log) {
    match log.last_recv() {
        Some(e) => println!("{:>4}. {}", e.index, e.text),
        None => println!("(no received message yet)"),
    }
}

fn print_last_sent(log: &Log) {
    match log.last_sent() {
        Some(e) => println!("{:>4}. {}", e.index, e.text),
        None => println!("(no sent message yet)"),
    }
}

async fn replay(client: &OcppClient, opts: &WriterOptions, log: &mut Log, index: usize) {
    let entry = log.find_sent_by_index(index).cloned();
    match entry {
        Some(e) => {
            let envelope = e.text.clone();
            match client.send_raw(&envelope).await {
                Ok(maybe_uid) => {
                    println!("{} replay #{}", opts.paint("↻", |s| s.cyan()), index);
                    println!("{}", writer::format_sent(&envelope, maybe_uid.as_deref(), opts));
                }
                Err(err) => {
                    println!("{} replay failed: {err}", opts.paint("✗", |s| s.red()));
                }
            }
        }
        None => {
            println!("no history entry with index {index}");
        }
    }
}

fn print_unknown(first_token: &str, opts: &WriterOptions) {
    println!(
        "{} Unknown command '{}'",
        opts.paint("✗", |s| s.red()),
        first_token
    );
    let suggestions = parser::suggest_similar(first_token);
    if !suggestions.is_empty() {
        println!();
        println!("Did you mean?");
        for (name, desc) in suggestions {
            println!("  • {:<18} {}", name, desc);
        }
    }
    println!();
    println!("Type 'help' to list all commands.");
}

fn print_help() {
    println!("charge_mgt_simulator — OCPP 1.6 REPL");
    println!();
    println!("CALLS:");
    println!("  <Action> [payload-json]       send a Call (auto uid)");
    println!("  /uid <id> <Action> [json]      send a Call with given uid");
    println!("  raw <envelope-json>            send raw envelope JSON as-is");
    println!();
    println!("RESPONSES TO SERVER CALLS:");
    println!("  respond <uid> <payload>  (r)  reply CallResult");
    println!("  error <uid> <code> <desc> (e) reply CallError");
    println!();
    println!("INSPECT:");
    println!("  status (st)                   connection + pending state");
    println!("  history (h)                   sent commands list");
    println!("  last (l)                      last received envelope");
    println!("  last-sent (ls)                last sent envelope");
    println!("  replay <n>                    re-send history item");
    println!();
    println!("HEARTBEAT:");
    println!("  heartbeat-on <secs> (hb)      auto-send Heartbeat every <secs>");
    println!("  heartbeat-off (hbo)           stop auto-heartbeat");
    println!();
    println!("OTHER:");
    println!("  help (?)                      this help");
    println!("  clear                         clear screen");
    println!("  quit (exit) / Ctrl+D          disconnect and exit");
    println!();
    println!("Commands (with aliases):");
    for (name, desc) in COMMAND_TABLE {
        println!("  {:<14} {}", name, desc);
    }
}
