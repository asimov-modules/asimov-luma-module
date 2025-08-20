// This is free and unencumbered software released into the public domain.

use super::Calendar;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FeaturedCalendar {
    pub is_subscriber: bool,
    pub is_admin: bool,
    pub membership_info: Option<serde_json::Value>,
    pub calendar: Calendar,
}
