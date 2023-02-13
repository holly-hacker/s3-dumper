use anyhow::Context;
use reqwest::{Client, Url};

use crate::models::ListObjectsV2Response;

#[derive(Default)]
pub struct S3Client {
    http: Client,
}

impl S3Client {
    pub async fn fetch(
        &self,
        url: &str,
        continuation_token: Option<&str>,
    ) -> anyhow::Result<ListObjectsV2Response> {
        let mut url = Url::parse(url).context("parse url")?;

        // TODO: add useful parameters (max-keys, prefix, delimiter)

        url.query_pairs_mut().append_pair("list-type", "2");

        if let Some(token) = continuation_token {
            url.query_pairs_mut()
                .append_pair("continuation-token", &token);
        }

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
