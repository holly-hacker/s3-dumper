use anyhow::Context;
use reqwest::{Client, Url};

use crate::models::ListObjectsV2Response;

#[derive(Default)]
pub struct S3Client {
    http: Client,
}

#[derive(Default)]
pub struct S3Options {
    pub max_keys: Option<usize>,
    pub prefix: Option<String>,
    pub delimiter: Option<String>,
}

impl S3Client {
    pub async fn fetch(
        &self,
        url: &str,
        options: &S3Options,
        continuation_token: Option<&str>,
    ) -> anyhow::Result<ListObjectsV2Response> {
        let mut url = Url::parse(url).context("parse url")?;

        url.query_pairs_mut().append_pair("list-type", "2");

        if let Some(token) = continuation_token {
            url.query_pairs_mut()
                .append_pair("continuation-token", token);
        }

        options.apply_to_url(&mut url);

        let response = self
            .http
            .get(url)
            .send()
            .await
            .context("download listing")?;
        if !response.status().is_success() {
            anyhow::bail!("received non-success response code: {}", response.status());
        }
        let bytes = response.bytes().await.context("download listing body")?;

        let parsed: ListObjectsV2Response =
            quick_xml::de::from_reader(&*bytes).context("deserialize response")?;

        Ok(parsed)
    }
}

impl S3Options {
    pub fn apply_to_url(&self, url: &mut Url) {
        let mut pairs = url.query_pairs_mut();

        if let Some(max_keys) = &self.max_keys {
            pairs.append_pair("max-keys", &max_keys.to_string());
        }

        if let Some(prefix) = &self.prefix {
            pairs.append_pair("prefix", prefix);
        }

        if let Some(delimiter) = &self.delimiter {
            pairs.append_pair("delimiter", delimiter);
        }
    }
}
