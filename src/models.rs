use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendNotificationRequest {
    pub title: Option<String>,
    pub body: Option<String>,
    pub big_title: Option<String>,
    pub big_description: Option<String>,
    pub big_summary_text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendNotificationResponse {
    pub value: Option<String>,
}
