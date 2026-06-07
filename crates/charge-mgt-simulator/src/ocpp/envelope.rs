use serde_json::Value;

#[derive(Debug, Clone)]
pub enum IncomingEnvelope {
    Call {
        uid: String,
        action: String,
        payload: Value,
    },
    CallResult {
        uid: String,
        payload: Value,
    },
    CallError {
        uid: String,
        code: String,
        description: String,
        details: Value,
    },
}

#[derive(Debug)]
pub struct EnvelopeError(pub String);

impl std::fmt::Display for EnvelopeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid envelope: {}", self.0)
    }
}

impl std::error::Error for EnvelopeError {}

impl IncomingEnvelope {
    pub fn parse(text: &str) -> Result<Self, EnvelopeError> {
        let v: Value =
            serde_json::from_str(text).map_err(|e| EnvelopeError(format!("not JSON: {e}")))?;
        let arr = v
            .as_array()
            .ok_or_else(|| EnvelopeError("not a JSON array".into()))?;
        let type_id = arr
            .first()
            .and_then(|x| x.as_i64())
            .ok_or_else(|| EnvelopeError("missing message_type_id".into()))?;
        match type_id {
            2 => {
                if arr.len() < 4 {
                    return Err(EnvelopeError("Call requires 4 elements".into()));
                }
                let uid = arr[1]
                    .as_str()
                    .ok_or_else(|| EnvelopeError("uid not a string".into()))?
                    .to_string();
                let action = arr[2]
                    .as_str()
                    .ok_or_else(|| EnvelopeError("action not a string".into()))?
                    .to_string();
                let payload = arr[3].clone();
                Ok(IncomingEnvelope::Call {
                    uid,
                    action,
                    payload,
                })
            }
            3 => {
                if arr.len() < 3 {
                    return Err(EnvelopeError("CallResult requires 3 elements".into()));
                }
                let uid = arr[1]
                    .as_str()
                    .ok_or_else(|| EnvelopeError("uid not a string".into()))?
                    .to_string();
                let payload = arr[2].clone();
                Ok(IncomingEnvelope::CallResult { uid, payload })
            }
            4 => {
                if arr.len() < 5 {
                    return Err(EnvelopeError("CallError requires 5 elements".into()));
                }
                let uid = arr[1]
                    .as_str()
                    .ok_or_else(|| EnvelopeError("uid not a string".into()))?
                    .to_string();
                let code = arr[2]
                    .as_str()
                    .ok_or_else(|| EnvelopeError("error_code not a string".into()))?
                    .to_string();
                let description = arr[3]
                    .as_str()
                    .ok_or_else(|| EnvelopeError("error_description not a string".into()))?
                    .to_string();
                let details = arr[4].clone();
                Ok(IncomingEnvelope::CallError {
                    uid,
                    code,
                    description,
                    details,
                })
            }
            other => Err(EnvelopeError(format!("unknown message_type_id={other}"))),
        }
    }
}

pub fn parse_raw_header(text: &str) -> Result<(i64, String, Option<String>), EnvelopeError> {
    let v: Value =
        serde_json::from_str(text).map_err(|e| EnvelopeError(format!("not JSON: {e}")))?;
    let arr = v
        .as_array()
        .ok_or_else(|| EnvelopeError("not a JSON array".into()))?;
    let type_id = arr
        .first()
        .and_then(|x| x.as_i64())
        .ok_or_else(|| EnvelopeError("missing message_type_id".into()))?;
    let uid = arr
        .get(1)
        .and_then(|x| x.as_str())
        .ok_or_else(|| EnvelopeError("missing/invalid uid".into()))?
        .to_string();
    let action = if type_id == 2 {
        arr.get(2).and_then(|x| x.as_str()).map(|s| s.to_string())
    } else {
        None
    };
    Ok((type_id, uid, action))
}
