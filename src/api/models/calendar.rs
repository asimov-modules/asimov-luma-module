// This is free and unencumbered software released into the public domain.

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Calendar {
    pub access_level: String,
    pub api_id: String,
    pub avatar_url: String,
    pub coordinate: Option<Coordinate>,
    pub cover_image_url: String,
    pub description_short: String,
    pub event_submission_restriction: String,
    pub geo_city: Option<String>,
    pub geo_country: Option<String>,
    pub geo_region: Option<String>,
    pub google_measurement_id: Option<String>,
    pub instagram_handle: Option<String>,
    pub is_blocked: bool,
    pub launch_status: String,
    pub linkedin_handle: Option<String>,
    pub luma_plus_active: bool,
    pub meta_pixel_id: Option<String>,
    pub name: String,
    pub personal_user_api_id: Option<String>,
    pub refund_policy: Option<RefundPolicy>,
    pub show_subscriber_count: bool,
    pub slug: String,
    pub social_image_url: Option<String>,
    pub stripe_account_id: Option<String>,
    pub tax_config: Option<serde_json::Value>,
    pub tiktok_handle: Option<String>,
    pub timezone: Option<String>,
    pub tint_color: Option<String>,
    pub track_meta_ads_from_luma: bool,
    pub twitter_handle: Option<String>,
    pub verified_at: String,
    pub website: String,
    pub youtube_handle: Option<String>,
    pub luma_featured_position: String,
    pub is_personal: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Coordinate {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefundPolicy {
    #[serde(rename = "type")]
    pub policy_type: String,
    pub content: Vec<PolicyContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PolicyContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub content: Vec<PolicyContentItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PolicyContentItem {
    pub text: String,
    #[serde(rename = "type")]
    pub item_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marks: Option<Vec<serde_json::Value>>,
}
