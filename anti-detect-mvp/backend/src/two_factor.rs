use base64::{engine::general_purpose, Engine as _};
use qrcode::render::svg;
use qrcode::QrCode;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use totp_lite::{totp_custom, Sha1};

const TOTP_DIGITS: u32 = 6;
const TOTP_STEP: u64 = 30;

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuth {
    pub secret: String,
    pub qr_code: String,
    pub backup_codes: Vec<String>,
}

pub fn generate_secret() -> String {
    let mut rng = rand::thread_rng();
    let secret: Vec<u8> = (0..20).map(|_| rng.gen()).collect();
    general_purpose::STANDARD.encode(secret)
}

pub fn generate_backup_codes(count: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    (0..count)
        .map(|_| {
            format!(
                "{:04}-{:04}-{:04}",
                rng.gen_range(0..10000),
                rng.gen_range(0..10000),
                rng.gen_range(0..10000)
            )
        })
        .collect()
}

pub fn generate_qr_code(
    secret: &str,
    username: &str,
    issuer: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let otpauth_url = format!(
        "otpauth://totp/{}:{}?secret={}&issuer={}",
        issuer, username, secret, issuer
    );

    let code = QrCode::new(otpauth_url.as_bytes())?;
    let svg = code.render::<svg::Color>().min_dimensions(200, 200).build();

    Ok(svg)
}

pub fn verify_totp(secret: &str, token: &str) -> bool {
    let secret_bytes = match general_purpose::STANDARD.decode(secret) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Check current time window and ±1 window for clock skew
    for i in -1..=1 {
        let time = timestamp as i64 + (i * TOTP_STEP as i64);
        if time < 0 {
            continue;
        }

        let expected = totp_custom::<Sha1>(TOTP_STEP, TOTP_DIGITS, &secret_bytes, time as u64);

        if token == expected {
            return true;
        }
    }

    false
}

pub fn setup_2fa(username: &str) -> Result<TwoFactorAuth, Box<dyn std::error::Error>> {
    let secret = generate_secret();
    let qr_code = generate_qr_code(&secret, username, "Anti-Detect Browser")?;
    let backup_codes = generate_backup_codes(8);

    Ok(TwoFactorAuth {
        secret,
        qr_code,
        backup_codes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_secret() {
        let secret = generate_secret();
        assert!(!secret.is_empty());
        assert!(general_purpose::STANDARD.decode(&secret).is_ok());
    }

    #[test]
    fn test_generate_backup_codes() {
        let codes = generate_backup_codes(8);
        assert_eq!(codes.len(), 8);
        for code in codes {
            assert_eq!(code.len(), 14); // XXXX-XXXX-XXXX
        }
    }

    #[test]
    fn test_verify_totp() {
        let secret = generate_secret();
        let secret_bytes = general_purpose::STANDARD.decode(&secret).unwrap();

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let token = totp_custom::<Sha1>(TOTP_STEP, TOTP_DIGITS, &secret_bytes, timestamp);

        assert!(verify_totp(&secret, &token));
        assert!(!verify_totp(&secret, "000000"));
    }

    #[test]
    fn test_setup_2fa() {
        let result = setup_2fa("test@example.com");
        assert!(result.is_ok());

        let tfa = result.unwrap();
        assert!(!tfa.secret.is_empty());
        assert!(!tfa.qr_code.is_empty());
        assert_eq!(tfa.backup_codes.len(), 8);
    }
}
