use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metrics {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub id: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    pub range: String,
    pub start: String,
    pub end: String,
    pub timeout: i64,
    #[serde(rename = "writes_only")]
    pub writes_only: bool,
    pub timezone: String,
    pub holidays: i64,
    pub status: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "modified_at")]
    pub modified_at: String,
    #[serde(rename = "days_minus_holidays")]
    pub days_minus_holidays: i64,
    pub languages: Vec<Language>,
    #[serde(rename = "daily_average_including_other_language")]
    pub daily_average_including_other_language: f64,
    #[serde(rename = "days_including_holidays")]
    pub days_including_holidays: i64,
    pub dependencies: Vec<Value>,
    #[serde(rename = "percent_calculated")]
    pub percent_calculated: i64,
    #[serde(rename = "human_readable_daily_average_including_other_language")]
    pub human_readable_daily_average_including_other_language: String,
    #[serde(rename = "total_seconds_including_other_language")]
    pub total_seconds_including_other_language: f64,
    #[serde(rename = "is_up_to_date")]
    pub is_up_to_date: bool,
    #[serde(rename = "is_up_to_date_pending_future")]
    pub is_up_to_date_pending_future: bool,
    #[serde(rename = "human_readable_total")]
    pub human_readable_total: String,
    #[serde(rename = "human_readable_total_including_other_language")]
    pub human_readable_total_including_other_language: String,
    #[serde(rename = "is_already_updating")]
    pub is_already_updating: bool,
    #[serde(rename = "human_readable_daily_average")]
    pub human_readable_daily_average: String,
    #[serde(rename = "total_seconds")]
    pub total_seconds: f64,
    #[serde(rename = "is_stuck")]
    pub is_stuck: bool,
    #[serde(rename = "daily_average")]
    pub daily_average: f64,
    #[serde(rename = "is_cached")]
    pub is_cached: bool,
    pub username: Value,
    #[serde(rename = "is_including_today")]
    pub is_including_today: bool,
    #[serde(rename = "human_readable_range")]
    pub human_readable_range: String,
    #[serde(rename = "is_coding_activity_visible")]
    pub is_coding_activity_visible: bool,
    #[serde(rename = "is_other_usage_visible")]
    pub is_other_usage_visible: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    pub name: String,
    #[serde(rename = "total_seconds")]
    pub total_seconds: f64,
    pub percent: f64,
    pub digital: String,
    pub decimal: String,
    pub text: String,
    pub hours: i64,
    pub minutes: i64,
}
