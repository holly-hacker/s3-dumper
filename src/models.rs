use chrono::{DateTime, FixedOffset};
use serde::{de::Visitor, Deserialize};

/// The root of this object is called `ListBucketResult`.
///
/// See also: https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListObjectsV2.html
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ListObjectsV2Response {
    pub is_truncated: bool,
    #[serde(default)]
    pub contents: Vec<Contents>,
    pub name: String,
    pub next_continuation_token: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Contents {
    pub key: String,
    pub size: usize,

    #[serde(deserialize_with = "deserialize_datetime_rfc3339")]
    pub last_modified: DateTime<FixedOffset>,
}

fn deserialize_datetime_rfc3339<'de, D>(d: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    d.deserialize_str(DateTimeVisitor)
}

struct DateTimeVisitor;

impl<'de> Visitor<'de> for DateTimeVisitor {
    type Value = DateTime<FixedOffset>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid RFC3339 datetime")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        DateTime::parse_from_rfc3339(v).map_err(|e| E::custom(e))
    }
}
