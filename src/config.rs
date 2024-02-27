use std::env;
use std::sync::Arc;
use std::str::FromStr;

pub use deadpool_postgres::{Pool};
use rustls::client::danger::{ServerCertVerified, ServerCertVerifier, HandshakeSignatureValid};
use rustls_pki_types::{ServerName, CertificateDer, UnixTime};
#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    // Configure SMTP for email.
    pub smtp_config: Option<SmtpConfig>,
}

#[derive(Clone, Debug)]
pub struct SmtpConfig {
    // Configure SMTP for email.
    pub host: String,
    pub port: u16,
    pub tls_off: bool,
    pub username: String,
    pub password: String,
}

impl SmtpConfig {
    pub fn new() -> Option<SmtpConfig> {
        let host = env::var("SMTP_HOST");
        let username = env::var("SMTP_USERNAME");
        let password = env::var("SMTP_PASSWORD");
        let smtp_port = env::var("SMTP_PORT");

        if let (Ok(host), Ok(username), Ok(password), Ok(smtp_port)) =
            (host, username, password, smtp_port)
        {
            Some(SmtpConfig {
                host,
                port: smtp_port.parse::<u16>().unwrap(),
                tls_off: env::var("SMTP_TLS_OFF").is_ok(),
                username,
                password,
            })
        } else {
            None
        }
    }
}

impl Config {
    // Initialise form oiur environment
    pub fn new() -> Config {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

        Config {
            database_url,
            smtp_config: SmtpConfig::new(),
        }
    }

    pub fn create_pool(&self) -> Pool {
        let config = tokio_postgres::Config::from_str(&self.database_url).unwrap();

        let manager = if config.get_ssl_mode() != tokio_postgres::config::SslMode::Disable {
            let tls_config = rustls::ClientConfig::builder()
                .dangerous()
                .with_custom_certificate_verifier(Arc::new(DummyTlsVerifier))
                .with_no_client_auth();

            let tls = tokio_postgres_rustls::MakeRustlsConnect::new(tls_config);
            deadpool_postgres::Manager::new(config, tls)
        } else {
            deadpool_postgres::Manager::new(config, tokio_postgres::NoTls)
        };

        Pool::builder(manager).build().unwrap()
    }
}

#[derive(Debug)]
struct DummyTlsVerifier;

impl ServerCertVerifier for DummyTlsVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer,
        _intermediates: &[CertificateDer],
        _server_name: &ServerName,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        Vec::new()
    }
}
