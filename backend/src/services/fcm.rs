use std::{
    sync::Arc,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::{info, warn, error};

#[derive(Debug, Deserialize)]
pub struct ServiceAccount {
    pub project_id: String,
    pub private_key: String,
    pub client_email: String,
    #[serde(default = "default_token_uri")]
    pub token_uri: String,
}

fn default_token_uri() -> String {
    "https://oauth2.googleapis.com/token".to_string()
}

#[derive(Serialize)]
struct JwtClaims {
    iss: String,
    scope: String,
    aud: String,
    iat: u64,
    exp: u64,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

struct CachedToken {
    token: String,
    expires_at: Instant,
}

struct FcmClientInner {
    account: ServiceAccount,
    http: reqwest::Client,
    cached: Mutex<Option<CachedToken>>,
}

/// FCM HTTP v1 client with automatic service account token refresh.
#[derive(Clone)]
pub struct FcmClient {
    inner: Arc<FcmClientInner>,
}

impl FcmClient {
    pub fn from_json(json: &str, http: reqwest::Client) -> Result<Self, serde_json::Error> {
        let account: ServiceAccount = serde_json::from_str(json)?;
        Ok(Self {
            inner: Arc::new(FcmClientInner {
                account,
                http,
                cached: Mutex::new(None),
            }),
        })
    }

    pub fn project_id(&self) -> &str {
        &self.inner.account.project_id
    }

    async fn access_token(&self) -> anyhow::Result<String> {
        let mut lock = self.inner.cached.lock().await;

        // Return cached token if it has more than 60 seconds left
        if let Some(ref c) = *lock {
            if c.expires_at > Instant::now() + Duration::from_secs(60) {
                return Ok(c.token.clone());
            }
        }

        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let claims = JwtClaims {
            iss: self.inner.account.client_email.clone(),
            scope: "https://www.googleapis.com/auth/firebase.messaging".to_string(),
            aud: self.inner.account.token_uri.clone(),
            iat: now,
            exp: now + 3600,
        };

        let key = EncodingKey::from_rsa_pem(self.inner.account.private_key.as_bytes())
            .map_err(|e| anyhow::anyhow!("Invalid RSA key: {e}"))?;

        let jwt = encode(&Header::new(Algorithm::RS256), &claims, &key)
            .map_err(|e| anyhow::anyhow!("JWT encode: {e}"))?;

        let resp: TokenResponse = self
            .inner
            .http
            .post(&self.inner.account.token_uri)
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("assertion", jwt.as_str()),
            ])
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        info!("🔑 FCM access token refreshed (expires in {}s)", resp.expires_in);

        *lock = Some(CachedToken {
            token: resp.access_token.clone(),
            expires_at: Instant::now() + Duration::from_secs(resp.expires_in),
        });

        Ok(resp.access_token)
    }

    /// Send a data-only FCM message so the Android app can format the text
    /// in the device locale and build a navigation intent.
    pub async fn send_alert_push(
        &self,
        tokens: &[String],
        reservoir_id: i32,
        reservoir_name: &str,
        condition_type: &str,
        threshold: f64,
        value: f64,
    ) {
        if tokens.is_empty() {
            return;
        }

        let access_token = match self.access_token().await {
            Ok(t) => t,
            Err(e) => {
                error!("FCM token exchange failed: {e}");
                return;
            }
        };

        let url = format!(
            "https://fcm.googleapis.com/v1/projects/{}/messages:send",
            self.project_id()
        );

        for token in tokens {
            let payload = serde_json::json!({
                "message": {
                    "token": token,
                    "data": {
                        "type": "alert",
                        "reservoir_id": reservoir_id.to_string(),
                        "reservoir_name": reservoir_name,
                        "condition_type": condition_type,
                        "threshold": threshold.to_string(),
                        "value": value.to_string(),
                    },
                    "android": { "priority": "HIGH" }
                }
            });

            match self
                .inner
                .http
                .post(&url)
                .bearer_auth(&access_token)
                .json(&payload)
                .send()
                .await
            {
                Ok(res) if res.status().is_success() => {
                    info!("🔔 Push sent to {}…", &token[..token.len().min(12)]);
                }
                Ok(res) => {
                    warn!("FCM error {}: {}", res.status(), res.text().await.unwrap_or_default());
                }
                Err(e) => {
                    error!("FCM request failed: {e:?}");
                }
            }
        }
    }
}
