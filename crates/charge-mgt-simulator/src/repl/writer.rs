use std::time::Duration;

use chrono::Local;
use colored::{ColoredString, Colorize};
use serde_json::Value;

use crate::ocpp::client::{IncomingEvent, IncomingMessage};

pub struct WriterOptions {
    pub no_color: bool,
}

impl WriterOptions {
    pub fn new(no_color: bool) -> Self {
        Self { no_color }
    }

    pub fn paint<T: ToString>(&self, text: T, f: fn(&str) -> ColoredString) -> String {
        let s = text.to_string();
        if self.no_color { s } else { f(&s).to_string() }
    }
}

pub fn format_incoming(ev: &IncomingEvent, opts: &WriterOptions) -> String {
    let ts = Local::now().format("%H:%M:%S").to_string();
    match ev {
        IncomingEvent::Message(msg) => format_message(msg, &ts, opts),
        IncomingEvent::BadEnvelope { raw, error } => {
            format!(
                "{ts} {} bad envelope: {error}\n  raw: {raw}",
                opts.paint("✗", |s| s.red())
            )
        }
        IncomingEvent::ConnectionClosed { reason } => {
            format!(
                "{ts} {} Connection closed: {reason}",
                opts.paint("🔌", |s| s.dimmed())
            )
        }
    }
}

fn format_message(msg: &IncomingMessage, ts: &str, opts: &WriterOptions) -> String {
    match msg {
        IncomingMessage::CallResult {
            uid,
            payload,
            matched_action,
            sent_ago,
        } => {
            let action_hint = matched_action.as_deref().unwrap_or("?");
            let ago_hint = sent_ago
                .map(format_duration)
                .unwrap_or_else(|| "n/a".into());
            let envelope = serde_json::json!([3, uid, payload]);
            format!(
                "{ts} {} ({} {ago_hint})\n{}",
                opts.paint("←", |s| s.blue()),
                opts.paint(action_hint, |s| s.dimmed()),
                pretty_envelope(&envelope, opts),
            )
        }
        IncomingMessage::CallError {
            uid,
            code,
            description,
            details,
            matched_action,
            sent_ago,
        } => {
            let action_hint = matched_action.as_deref().unwrap_or("?");
            let ago_hint = sent_ago
                .map(format_duration)
                .unwrap_or_else(|| "n/a".into());
            let envelope = serde_json::json!([4, uid, code, description, details]);
            format!(
                "{ts} {} {} \"{}\" ({} {ago_hint})\n{}",
                opts.paint("✗", |s| s.red()),
                opts.paint(code, |s| s.red().bold()),
                description,
                opts.paint(action_hint, |s| s.dimmed()),
                pretty_envelope(&envelope, opts),
            )
        }
        IncomingMessage::ServerCall {
            uid,
            action,
            payload,
        } => {
            let envelope = serde_json::json!([2, uid, action, payload]);
            format!(
                "{ts} {} {} from server\n{}\n  → respond {uid} {{...}}  or  error {uid} <code> <desc>",
                opts.paint("⚡", |s| s.yellow()),
                opts.paint(action, |s| s.yellow().bold()),
                pretty_envelope(&envelope, opts),
            )
        }
    }
}

pub fn format_sent(envelope_text: &str, uid: Option<&str>, opts: &WriterOptions) -> String {
    let ts = Local::now().format("%H:%M:%S").to_string();
    let body = if let Ok(v) = serde_json::from_str::<Value>(envelope_text) {
        pretty_envelope(&v, opts)
    } else {
        envelope_text.to_string()
    };
    let uid_hint = uid.map(|u| format!(" uid={u}")).unwrap_or_default();
    format!("{ts} {}{uid_hint}\n{body}", opts.paint("→", |s| s.green()))
}

pub fn pretty_envelope(v: &Value, opts: &WriterOptions) -> String {
    let pretty = serde_json::to_string_pretty(v).unwrap_or_else(|_| v.to_string());
    if opts.no_color {
        return pretty;
    }
    // 仅做最小着色：字符串用浅青，数字用黄，关键词用洋红
    // 为避免破坏缩进，仅对整体做粗粒度替换
    pretty
}

fn format_duration(d: Duration) -> String {
    let ms = d.as_millis();
    if ms < 1000 {
        format!("{ms}ms")
    } else {
        format!("{:.2}s", ms as f64 / 1000.0)
    }
}
