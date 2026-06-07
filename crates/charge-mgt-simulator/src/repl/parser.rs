use serde_json::{self, Value};
use strsim::levenshtein;

#[derive(Debug, Clone)]
pub enum ReplCommand {
    Help,
    Quit,
    Status,
    Clear,
    History,
    Last,
    LastSent,
    Replay { index: usize },
    HeartbeatOn { interval_secs: u64 },
    HeartbeatOff,
    SendRaw { envelope_text: String },
    SendCall { action: String, payload: Value, uid: Option<String> },
    Respond { uid: String, payload: Value },
    SendError { uid: String, code: String, description: String },
    Unknown { first_token: String },
}

pub const COMMAND_TABLE: &[(&str, &str)] = &[
    ("help",          "show help"),
    ("?",             "show help"),
    ("quit",          "close connection and exit"),
    ("exit",          "close connection and exit"),
    ("status",        "show connection & pending state"),
    ("st",            "status (shortcut)"),
    ("clear",         "clear screen"),
    ("history",       "list past user inputs"),
    ("h",             "history (shortcut)"),
    ("last",          "print last received message"),
    ("l",             "last (shortcut)"),
    ("last-sent",     "print last sent message"),
    ("ls",            "last-sent (shortcut)"),
    ("replay",        "replay history item by index"),
    ("raw",           "send raw envelope JSON verbatim"),
    ("respond",       "reply to server call with CallResult"),
    ("r",             "respond (shortcut)"),
    ("error",         "reply to server call with CallError"),
    ("e",             "error (shortcut)"),
    ("heartbeat-on",  "start auto-heartbeat task"),
    ("hb",            "heartbeat-on (shortcut)"),
    ("heartbeat-off", "stop auto-heartbeat task"),
    ("hbo",           "heartbeat-off (shortcut)"),
];

pub fn parse(line: &str) -> Option<ReplCommand> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }

    // slash-prefix 命令: /uid <id> <action> [payload]
    if let Some(rest) = trimmed.strip_prefix('/') {
        let rest = rest.trim_start();
        if let Some(rem) = rest.strip_prefix("uid ") {
            return parse_uid_command(rem.trim_start());
        }
        // 其他 slash 命令当未知处理
        let first = rest.split_whitespace().next().unwrap_or("").to_string();
        return Some(ReplCommand::Unknown { first_token: format!("/{first}") });
    }

    // raw JSON envelope
    if trimmed.starts_with('[') || trimmed.starts_with('{') {
        return Some(ReplCommand::SendRaw { envelope_text: trimmed.to_string() });
    }

    // tokenized command
    let parts = trim_split_first_ws(trimmed);
    let first = parts.0.to_string();
    let rest = parts.1;

    match first.as_str() {
        "help" | "?" => Some(ReplCommand::Help),
        "quit" | "exit" => Some(ReplCommand::Quit),
        "status" | "st" => Some(ReplCommand::Status),
        "clear" => Some(ReplCommand::Clear),
        "history" | "h" => Some(ReplCommand::History),
        "last" | "l" => Some(ReplCommand::Last),
        "last-sent" | "ls" => Some(ReplCommand::LastSent),
        "heartbeat-off" | "hbo" => Some(ReplCommand::HeartbeatOff),
        "replay" => parse_replay(rest),
        "raw" => parse_raw(rest),
        "respond" | "r" => parse_respond(rest),
        "error" | "e" => parse_error(rest),
        "heartbeat-on" | "hb" => parse_heartbeat_on(rest),
        _ => {
            // 可能是 shorthand Call
            if looks_like_action(&first) {
                return parse_shorthand_call(first, rest);
            }
            Some(ReplCommand::Unknown { first_token: first })
        }
    }
}

fn parse_uid_command(rest: &str) -> Option<ReplCommand> {
    // 格式: <uid> <action> [payload]
    let (uid, rest) = trim_split_first_ws(rest);
    if uid.is_empty() {
        return Some(ReplCommand::Unknown { first_token: "/uid".to_string() });
    }
    let (action, rest) = trim_split_first_ws(rest);
    if action.is_empty() {
        return Some(ReplCommand::Unknown { first_token: "/uid".to_string() });
    }
    let payload = parse_optional_json(rest).unwrap_or(Value::Object(serde_json::Map::new()));
    Some(ReplCommand::SendCall {
        action: action.to_string(),
        payload,
        uid: Some(uid.to_string()),
    })
}

fn parse_replay(rest: &str) -> Option<ReplCommand> {
    let rest = rest.trim();
    match rest.parse::<usize>() {
        Ok(i) if i >= 1 => Some(ReplCommand::Replay { index: i }),
        _ => Some(ReplCommand::Unknown { first_token: "replay".to_string() }),
    }
}

fn parse_raw(rest: &str) -> Option<ReplCommand> {
    let rest = rest.trim();
    if rest.is_empty() {
        return Some(ReplCommand::Unknown { first_token: "raw".to_string() });
    }
    Some(ReplCommand::SendRaw { envelope_text: rest.to_string() })
}

fn parse_respond(rest: &str) -> Option<ReplCommand> {
    let (uid, rest) = trim_split_first_ws(rest);
    if uid.is_empty() {
        return Some(ReplCommand::Unknown { first_token: "respond".to_string() });
    }
    let payload = parse_optional_json(rest).unwrap_or(Value::Object(serde_json::Map::new()));
    Some(ReplCommand::Respond { uid: uid.to_string(), payload })
}

fn parse_error(rest: &str) -> Option<ReplCommand> {
    // error <uid> <code> <description>
    let (uid, rest) = trim_split_first_ws(rest);
    let (code, rest) = trim_split_first_ws(rest);
    let description = rest.trim();
    if uid.is_empty() || code.is_empty() || description.is_empty() {
        return Some(ReplCommand::Unknown { first_token: "error".to_string() });
    }
    Some(ReplCommand::SendError {
        uid: uid.to_string(),
        code: code.to_string(),
        description: description.to_string(),
    })
}

fn parse_heartbeat_on(rest: &str) -> Option<ReplCommand> {
    let rest = rest.trim();
    match rest.parse::<u64>() {
        Ok(s) if s > 0 => Some(ReplCommand::HeartbeatOn { interval_secs: s }),
        _ => Some(ReplCommand::Unknown { first_token: "heartbeat-on".to_string() }),
    }
}

fn parse_shorthand_call(action: String, rest: &str) -> Option<ReplCommand> {
    let payload = parse_optional_json(rest).unwrap_or(Value::Object(serde_json::Map::new()));
    Some(ReplCommand::SendCall { action, payload, uid: None })
}

fn parse_optional_json(text: &str) -> Option<Value> {
    let text = text.trim();
    if text.is_empty() {
        return None;
    }
    serde_json::from_str(text).ok()
}

fn looks_like_action(s: &str) -> bool {
    // OCPP 动作名是 PascalCase（首字母大写），与 REPL 命令（全小写）形成正交命名空间
    !s.is_empty()
        && s.chars().next().map(|c| c.is_ascii_uppercase()).unwrap_or(false)
        && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn trim_split_first_ws(s: &str) -> (&str, &str) {
    let s = s.trim_start();
    match s.find(|c: char| c.is_ascii_whitespace()) {
        Some(i) => (&s[..i], s[i..].trim_start()),
        None => (s, ""),
    }
}

/// 给出与输入最相似的 (命令名, 描述) 候选列表。
pub fn suggest_similar(input: &str) -> Vec<(&'static str, &'static str)> {
    let input_lower = input.to_ascii_lowercase();
    let mut out: Vec<(&'static str, &'static str, usize)> = COMMAND_TABLE
        .iter()
        .filter_map(|(name, desc)| {
            if input_lower.len() < 3 {
                return None;
            }
            let name_lower = name.to_ascii_lowercase();
            let shared_prefix: usize = input_lower
                .chars()
                .zip(name_lower.chars())
                .take(3)
                .take_while(|(a, b)| a == b)
                .count();
            if shared_prefix < 3 {
                return None;
            }
            let dist = levenshtein(&input_lower, &name_lower);
            if dist == 0 || dist > 2 {
                return None;
            }
            Some((*name, *desc, dist))
        })
        .collect();
    out.sort_by_key(|(_, _, d)| *d);
    out.into_iter().map(|(n, d, _)| (n, d)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_shorthand() {
        match parse("Heartbeat {}") {
            Some(ReplCommand::SendCall { action, uid, .. }) => {
                assert_eq!(action, "Heartbeat");
                assert!(uid.is_none());
            }
            x => panic!("expected SendCall, got {x:?}"),
        }
    }

    #[test]
    fn parse_raw() {
        match parse(r#"raw [2, "x", "Heartbeat", {}]"#) {
            Some(ReplCommand::SendRaw { .. }) => {}
            x => panic!("got {x:?}"),
        }
    }

    #[test]
    fn parse_json_envelope() {
        match parse(r#"[2, "x", "Heartbeat", {}]"#) {
            Some(ReplCommand::SendRaw { .. }) => {}
            x => panic!("got {x:?}"),
        }
    }

    #[test]
    fn parse_slash_uid() {
        match parse("/uid my-id BootNotification {}") {
            Some(ReplCommand::SendCall { action, uid, .. }) => {
                assert_eq!(action, "BootNotification");
                assert_eq!(uid.as_deref(), Some("my-id"));
            }
            x => panic!("got {x:?}"),
        }
    }

    #[test]
    fn parse_respond() {
        match parse(r#"respond s-1 {"status":"Accepted"}"#) {
            Some(ReplCommand::Respond { uid, .. }) => assert_eq!(uid, "s-1"),
            x => panic!("got {x:?}"),
        }
    }

    #[test]
    fn parse_unknown() {
        match parse("statu") {
            Some(ReplCommand::Unknown { first_token }) => assert_eq!(first_token, "statu"),
            x => panic!("got {x:?}"),
        }
    }

    #[test]
    fn empty_skipped() {
        assert!(parse("   ").is_none());
        assert!(parse("").is_none());
    }

    #[test]
    fn suggestions_for_typo() {
        let s = suggest_similar("statu");
        let names: Vec<_> = s.iter().map(|(n, _)| *n).collect();
        assert!(names.contains(&"status"), "suggestions = {names:?}");
    }
}
