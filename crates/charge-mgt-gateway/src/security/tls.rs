//! TLS 配置加载（rustls 纯 Rust 实现，避免 OpenSSL 系统依赖）
//!
//! 支持 4 种模式：
//! - `load_server_config`：模式 1-4 单向 TLS
//! - `load_mtls_server_config`：模式 5/6 mTLS（双向证书认证）

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

use rustls::RootCertStore;
use rustls::ServerConfig;
use rustls::pki_types::{PrivateKeyDer, PrivatePkcs8KeyDer};
use rustls::server::WebPkiClientVerifier;
use rustls_pemfile::{certs, pkcs8_private_keys};

use crate::config::MtlsConfig;
use crate::error::GatewayError;

/// 从 PEM 文件加载服务端 TLS 配置
///
/// - `cert_path`：PEM 格式证书链
/// - `key_path`：PEM 格式 PKCS#8 私钥
pub fn load_server_config(cert_path: &Path, key_path: &Path) -> Result<Arc<ServerConfig>, GatewayError> {
    let certs = read_certs(cert_path)?;
    let key = read_key(key_path)?;

    let config = ServerConfig::builder()
        .with_no_client_auth() // 单向认证：不要求桩提供证书
        .with_single_cert(certs, PrivateKeyDer::Pkcs8(key))
        .map_err(|e| GatewayError::Tls(format!("构建 ServerConfig 失败: {e}")))?;

    Ok(Arc::new(config))
}

/// 从 PEM 文件加载服务端 mTLS 配置（双向：CA 验证客户端证书）
///
/// - `cert_path` / `key_path`：服务端自己的证书 + 私钥
/// - `mtls.ca_cert_path`：信任的 CA 证书池（桩客户端证书必须由其中之一签发）
pub fn load_mtls_server_config(
    cert_path: &Path,
    key_path: &Path,
    mtls: &MtlsConfig,
) -> Result<Arc<ServerConfig>, GatewayError> {
    let server_certs = read_certs(cert_path)?;
    let key = read_key(key_path)?;

    // 加载 CA 证书池
    let mut root_store = RootCertStore::empty();
    let ca_file = File::open(&mtls.ca_cert_path).map_err(|e| {
        GatewayError::Tls(format!(
            "打开 CA 证书失败 {}: {e}",
            mtls.ca_cert_path.display()
        ))
    })?;
    for ca in certs(&mut BufReader::new(ca_file))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| GatewayError::Tls(format!("解析 CA 证书失败: {e}")))?
    {
        root_store.add(ca).map_err(|e| GatewayError::Tls(e.to_string()))?;
    }

    // 构造 client cert verifier
    let verifier = WebPkiClientVerifier::builder(root_store.into())
        .build()
        .map_err(|e| GatewayError::Tls(format!("构建 ClientCertVerifier 失败: {e}")))?;

    let config = ServerConfig::builder()
        .with_client_cert_verifier(verifier)
        .with_single_cert(server_certs, PrivateKeyDer::Pkcs8(key))
        .map_err(|e| GatewayError::Tls(format!("构建 mTLS ServerConfig 失败: {e}")))?;

    Ok(Arc::new(config))
}

fn read_certs(path: &Path) -> Result<Vec<rustls::pki_types::CertificateDer<'static>>, GatewayError> {
    let file = File::open(path)
        .map_err(|e| GatewayError::Tls(format!("打开证书失败 {}: {e}", path.display())))?;
    certs(&mut BufReader::new(file))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| GatewayError::Tls(format!("解析证书失败: {e}")))
}

fn read_key(path: &Path) -> Result<PrivatePkcs8KeyDer<'static>, GatewayError> {
    let file = File::open(path)
        .map_err(|e| GatewayError::Tls(format!("打开私钥失败 {}: {e}", path.display())))?;
    pkcs8_private_keys(&mut BufReader::new(file))
        .next()
        .ok_or_else(|| GatewayError::Tls("私钥文件为空".into()))?
        .map_err(|e| GatewayError::Tls(format!("解析私钥失败: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 集成测试：需要系统有 openssl。CI 环境通常有；开发机也通常有。
    /// 失败时跳过（不 panic），避免依赖项缺失导致 cargo test 全红。
    #[test]
    fn loads_valid_certs() {
        let Some((cert, key)) = generate_test_certs() else {
            eprintln!("skip: openssl not available");
            return;
        };
        let cfg = load_server_config(&cert, &key);
        assert!(cfg.is_ok(), "{:?}", cfg.err());
    }

    fn generate_test_certs() -> Option<(std::path::PathBuf, std::path::PathBuf)> {
        let dir = std::env::temp_dir().join(format!("tls_test_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let cert = dir.join("cert.pem");
        let key = dir.join("key.pem");

        let out = std::process::Command::new("openssl")
            .args(["req", "-x509", "-newkey", "rsa:2048", "-keyout"])
            .arg(&key)
            .args(["-out"])
            .arg(&cert)
            .args(["-days", "1", "-nodes", "-subj", "/CN=test"])
            .output()
            .ok()?;
        if !out.status.success() {
            return None;
        }
        Some((cert, key))
    }
}