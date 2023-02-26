use serde::{Serialize, Deserialize};
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
pub struct GrafanaWebhook {
    #[serde(rename = "receiver")]
    pub receiver: String,

    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "orgId")]
    pub org_id: i64,

    #[serde(rename = "alerts")]
    pub alerts: Vec<Alert>,

    #[serde(rename = "groupLabels")]
    pub group_labels: CommonAnnotations,

    #[serde(rename = "commonLabels")]
    pub common_labels: CommonLabels,

    #[serde(rename = "commonAnnotations")]
    pub common_annotations: CommonAnnotations,

    #[serde(rename = "externalURL")]
    pub external_url: String,

    #[serde(rename = "version")]
    pub version: String,

    #[serde(rename = "groupKey")]
    pub group_key: String,

    #[serde(rename = "truncatedAlerts")]
    pub truncated_alerts: i64,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "state")]
    pub state: String,

    #[serde(rename = "message")]
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Alert {
    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "labels")]
    pub labels: Labels,

    #[serde(rename = "annotations")]
    pub annotations: Annotations,

    #[serde(rename = "startsAt")]
    pub starts_at: String,

    #[serde(rename = "endsAt")]
    pub ends_at: String,

    #[serde(rename = "generatorURL")]
    pub generator_url: String,

    #[serde(rename = "fingerprint")]
    pub fingerprint: String,

    #[serde(rename = "silenceURL")]
    pub silence_url: String,

    #[serde(rename = "dashboardURL")]
    pub dashboard_url: String,

    #[serde(rename = "panelURL")]
    pub panel_url: String,

    #[serde(rename = "valueString")]
    pub value_string: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Annotations {
    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "runbook_url")]
    pub runbook_url: String,

    #[serde(rename = "summary")]
    pub summary: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Labels {
    #[serde(rename = "alertname")]
    pub alertname: String,

    #[serde(rename = "team")]
    pub team: String,

    #[serde(rename = "zone")]
    pub zone: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommonAnnotations {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommonLabels {
    #[serde(rename = "team")]
    pub team: String,
}
