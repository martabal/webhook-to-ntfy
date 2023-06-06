use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrafanaWebhook {
    pub receiver: String,
    pub status: String,
    pub alerts: Vec<Alert>,
    pub group_labels: GroupLabels,
    pub common_labels: CommonLabels,
    pub common_annotations: CommonAnnotations,
    #[serde(rename = "externalURL")]
    pub external_url: String,
    pub version: String,
    pub group_key: String,
    pub truncated_alerts: i64,
    pub org_id: i64,
    pub title: String,
    pub state: String,
    pub message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    pub status: String,
    pub labels: Labels,
    pub annotations: Annotations,
    pub starts_at: String,
    pub ends_at: String,
    #[serde(rename = "generatorURL")]
    pub generator_url: String,
    pub fingerprint: String,
    #[serde(rename = "silenceURL")]
    pub silence_url: String,
    #[serde(rename = "dashboardURL")]
    pub dashboard_url: String,
    #[serde(rename = "panelURL")]
    pub panel_url: String,
    pub value_string: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Labels {

    pub instance: Option<String>,
    pub alertname: String,
    #[serde(rename = "grafana_folder")]
    pub grafana_folder: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotations {
    pub summary: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupLabels {
    pub alertname: Option<String>,
    #[serde(rename = "grafana_folder")]
    pub grafana_folder: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonLabels {
    pub alertname: String,
    pub instance: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonAnnotations {
    pub summary: Option<String>,
}
