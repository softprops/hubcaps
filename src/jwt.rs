use crate::errors::Result;
use jsonwebtoken as jwt;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::time;

pub use jsonwebtoken::errors;

// We use 9 minutes for the life to give some buffer for clock drift between
// our clock and GitHub's. The absolute max is 10 minutes.
const MAX_JWT_TOKEN_LIFE: time::Duration = time::Duration::from_secs(60 * 9);

// 8 minutes so we refresh sooner than it actually expires
const JWT_TOKEN_REFRESH_PERIOD: time::Duration = time::Duration::from_secs(60 * 8);

/// JSON Web Token authentication mechanism
///
/// The GitHub client methods are all &self, but the dynamically
/// generated JWT token changes regularly. The token is also a bit
/// expensive to regenerate, so we do want to have a mutable cache.
///
/// We use a token inside a Mutex so we can have interior mutability
/// even though JWTCredentials is not mutable.
#[derive(Clone)]
pub struct JWTCredentials {
    pub app_id: u64,
    /// DER RSA key. Generate with
    /// `openssl rsa -in private_rsa_key.pem -outform DER -out private_rsa_key.der`
    pub private_key: Vec<u8>,
    cache: Arc<Mutex<ExpiringJWTCredential>>,
}

impl JWTCredentials {
    pub fn new(app_id: u64, private_key: Vec<u8>) -> Result<JWTCredentials> {
        let creds = ExpiringJWTCredential::calculate(app_id, &private_key)?;

        Ok(JWTCredentials {
            app_id,
            private_key,
            cache: Arc::new(Mutex::new(creds)),
        })
    }

    #[cfg(feature = "app")]
    pub(crate) fn is_stale(&self) -> bool {
        self.cache.lock().unwrap().is_stale()
    }

    /// Fetch a valid JWT token, regenerating it if necessary
    pub fn token(&self) -> String {
        let mut expiring = self.cache.lock().unwrap();
        if expiring.is_stale() {
            *expiring = ExpiringJWTCredential::calculate(self.app_id, &self.private_key)
                .expect("JWT private key worked before, it should work now...");
        }

        expiring.token.clone()
    }
}

impl PartialEq for JWTCredentials {
    fn eq(&self, other: &JWTCredentials) -> bool {
        self.app_id == other.app_id && self.private_key == other.private_key
    }
}

#[derive(Debug)]
struct ExpiringJWTCredential {
    token: String,
    created_at: time::Instant,
}

#[derive(Serialize)]
struct JWTCredentialClaim {
    iat: u64,
    exp: u64,
    iss: u64,
}

impl ExpiringJWTCredential {
    fn calculate(app_id: u64, private_key: &[u8]) -> Result<ExpiringJWTCredential> {
        // SystemTime can go backwards, Instant can't, so always use
        // Instant for ensuring regular cycling.
        let created_at = time::Instant::now();
        let now = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap();
        let expires = now + MAX_JWT_TOKEN_LIFE;

        let payload = JWTCredentialClaim {
            iat: now.as_secs(),
            exp: expires.as_secs(),
            iss: app_id,
        };
        let header = jwt::Header::new(jwt::Algorithm::RS256);
        let jwt = jwt::encode(
            &header,
            &payload,
            &jsonwebtoken::EncodingKey::from_rsa_der(private_key),
        )?;

        Ok(ExpiringJWTCredential {
            created_at,
            token: jwt,
        })
    }

    fn is_stale(&self) -> bool {
        self.created_at.elapsed() >= JWT_TOKEN_REFRESH_PERIOD
    }
}
