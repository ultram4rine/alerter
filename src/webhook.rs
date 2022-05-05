use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Representation of `webhook` from Alertmanager.
///
/// See https://prometheus.io/docs/alerting/latest/configuration/#webhook_config.
#[derive(Serialize, Deserialize)]
pub struct WebHook {
    pub status: String,
    pub alerts: Vec<Alert>,
}

/// Representation of `alert` from Alertmanager.
///
/// See https://prometheus.io/docs/alerting/latest/configuration/#webhook_config.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    pub labels: Value,
    pub annotations: Value,
    pub starts_at: String,
    pub ends_at: String,
}
