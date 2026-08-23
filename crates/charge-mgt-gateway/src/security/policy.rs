//! OCPP 1.6 链路安全策略。
//!
//! `SecurityMode` 是从配置推导出的内部枚举；
//! 启动时校验 config 组合合法（6 种）。

use crate::config::{AuthMode, OcppSecurityConfig, TlsConfig};
use crate::error::GatewayError;

/// OCPP 1.6 链路安全模式（6 种）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityMode {
    /// 模式 1：明文 ws://，无认证
    NoTlsNoAuth,
    /// 模式 2：明文 ws:// + Basic Auth
    BasicNoTls,
    /// 模式 3：wss://，无认证
    TlsOnly,
    /// 模式 4：wss:// + Basic Auth（OCPP 1.6 主流）
    TlsWithBasic,
    /// 模式 5：wss:// + mTLS（双向证书认证）
    TlsWithMtls,
    /// 模式 6：wss:// + mTLS + Basic Auth（最高安全）
    TlsWithMtlsAndBasic,
}

impl SecurityMode {
    /// 从配置推导模式，启动时校验合法性
    pub fn from_config(cfg: &OcppSecurityConfig) -> Result<Self, GatewayError> {
        let tls = cfg.tls.enabled;
        let mtls = cfg.tls.mtls.as_ref().map(|m| m.ca_cert_path.exists()).unwrap_or(false);
        let m = match (&cfg.auth_mode, tls, mtls) {
            (AuthMode::None, false, _) => Self::NoTlsNoAuth,
            (AuthMode::None, true, false) => Self::TlsOnly,
            (AuthMode::Basic, false, _) => Self::BasicNoTls,
            (AuthMode::Basic, true, false) => Self::TlsWithBasic,
            (AuthMode::Mtls, true, true) => Self::TlsWithMtls,
            (AuthMode::MtlsWithBasic, true, true) => Self::TlsWithMtlsAndBasic,
            (mode, tls_on, mtls_on) => {
                return Err(GatewayError::Config(format!(
                    "非法安全配置: auth_mode={mode:?}, tls={tls_on}, mtls={mtls_on}"
                )));
            }
        };

        if tls {
            validate_tls_paths(&cfg.tls)?;
        }
        Ok(m)
    }

    /// 对外 URI scheme（`ws` 或 `wss`）
    pub fn scheme(&self) -> &'static str {
        match self {
            Self::NoTlsNoAuth | Self::BasicNoTls => "ws",
            Self::TlsOnly
            | Self::TlsWithBasic
            | Self::TlsWithMtls
            | Self::TlsWithMtlsAndBasic => "wss",
        }
    }

    /// 模式是否启用 TLS
    pub fn is_tls(&self) -> bool {
        !matches!(self, Self::NoTlsNoAuth | Self::BasicNoTls)
    }

    /// 模式是否要求 mTLS（双向证书认证）
    pub fn requires_mtls(&self) -> bool {
        matches!(self, Self::TlsWithMtls | Self::TlsWithMtlsAndBasic)
    }

    /// 模式是否要求 Basic Auth
    pub fn requires_basic(&self) -> bool {
        matches!(
            self,
            Self::BasicNoTls | Self::TlsWithBasic | Self::TlsWithMtlsAndBasic
        )
    }
}

fn validate_tls_paths(tls: &TlsConfig) -> Result<(), GatewayError> {
    if tls.cert_path.is_none() || tls.key_path.is_none() {
        return Err(GatewayError::Config(
            "tls.enabled=true 但 cert_path/key_path 缺失".into(),
        ));
    }
    if !tls.cert_path.as_ref().unwrap().exists() {
        return Err(GatewayError::Config(format!(
            "证书文件不存在: {}",
            tls.cert_path.as_ref().unwrap().display()
        )));
    }
    if !tls.key_path.as_ref().unwrap().exists() {
        return Err(GatewayError::Config(format!(
            "私钥文件不存在: {}",
            tls.key_path.as_ref().unwrap().display()
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::MtlsConfig;
    use std::path::PathBuf;

    fn cfg(auth: AuthMode, tls: bool, cert: bool, key: bool) -> OcppSecurityConfig {
        OcppSecurityConfig {
            auth_mode: auth,
            tls: TlsConfig {
                enabled: tls,
                cert_path: if cert { Some(PathBuf::from("/dev/null")) } else { None },
                key_path: if key { Some(PathBuf::from("/dev/null")) } else { None },
                mtls: None,
            },
        }
    }

    fn cfg_mtls(auth: AuthMode, with_ca: bool) -> OcppSecurityConfig {
        OcppSecurityConfig {
            auth_mode: auth,
            tls: TlsConfig {
                enabled: true,
                cert_path: Some(PathBuf::from("/dev/null")),
                key_path: Some(PathBuf::from("/dev/null")),
                mtls: with_ca.then(|| MtlsConfig {
                    ca_cert_path: PathBuf::from("/dev/null"),
                    client_auth: Default::default(),
                }),
            },
        }
    }

    #[test]
    fn mode1_no_tls_no_auth() {
        assert_eq!(
            SecurityMode::from_config(&cfg(AuthMode::None, false, false, false)).unwrap(),
            SecurityMode::NoTlsNoAuth
        );
    }

    #[test]
    fn mode2_basic_no_tls() {
        assert_eq!(
            SecurityMode::from_config(&cfg(AuthMode::Basic, false, false, false)).unwrap(),
            SecurityMode::BasicNoTls
        );
    }

    #[test]
    fn mode3_tls_no_auth() {
        assert_eq!(
            SecurityMode::from_config(&cfg(AuthMode::None, true, true, true)).unwrap(),
            SecurityMode::TlsOnly
        );
    }

    #[test]
    fn mode4_tls_with_basic() {
        assert_eq!(
            SecurityMode::from_config(&cfg(AuthMode::Basic, true, true, true)).unwrap(),
            SecurityMode::TlsWithBasic
        );
    }

    #[test]
    fn mode5_tls_with_mtls() {
        assert_eq!(
            SecurityMode::from_config(&cfg_mtls(AuthMode::Mtls, true)).unwrap(),
            SecurityMode::TlsWithMtls
        );
    }

    #[test]
    fn mode6_tls_with_mtls_and_basic() {
        assert_eq!(
            SecurityMode::from_config(&cfg_mtls(AuthMode::MtlsWithBasic, true)).unwrap(),
            SecurityMode::TlsWithMtlsAndBasic
        );
    }

    #[test]
    fn mtls_requires_ca() {
        // mtls 模式但没配 ca_cert_path → 非法
        let e = SecurityMode::from_config(&cfg_mtls(AuthMode::Mtls, false)).unwrap_err();
        assert!(format!("{e}").contains("非法安全配置"));
    }

    #[test]
    fn tls_requires_cert_and_key() {
        let e = SecurityMode::from_config(&cfg(AuthMode::None, true, false, true)).unwrap_err();
        assert!(format!("{e}").contains("cert_path"));
        let e = SecurityMode::from_config(&cfg(AuthMode::None, true, true, false)).unwrap_err();
        assert!(format!("{e}").contains("key_path"));
    }

    #[test]
    fn scheme_per_mode() {
        assert_eq!(SecurityMode::NoTlsNoAuth.scheme(), "ws");
        assert_eq!(SecurityMode::BasicNoTls.scheme(), "ws");
        assert_eq!(SecurityMode::TlsOnly.scheme(), "wss");
        assert_eq!(SecurityMode::TlsWithBasic.scheme(), "wss");
        assert_eq!(SecurityMode::TlsWithMtls.scheme(), "wss");
        assert_eq!(SecurityMode::TlsWithMtlsAndBasic.scheme(), "wss");
    }

    #[test]
    fn requires_basic_per_mode() {
        assert!(!SecurityMode::NoTlsNoAuth.requires_basic());
        assert!(SecurityMode::BasicNoTls.requires_basic());
        assert!(!SecurityMode::TlsOnly.requires_basic());
        assert!(SecurityMode::TlsWithBasic.requires_basic());
        assert!(!SecurityMode::TlsWithMtls.requires_basic());
        assert!(SecurityMode::TlsWithMtlsAndBasic.requires_basic());
    }

    #[test]
    fn requires_mtls_per_mode() {
        assert!(!SecurityMode::NoTlsNoAuth.requires_mtls());
        assert!(!SecurityMode::BasicNoTls.requires_mtls());
        assert!(!SecurityMode::TlsOnly.requires_mtls());
        assert!(!SecurityMode::TlsWithBasic.requires_mtls());
        assert!(SecurityMode::TlsWithMtls.requires_mtls());
        assert!(SecurityMode::TlsWithMtlsAndBasic.requires_mtls());
    }
}
