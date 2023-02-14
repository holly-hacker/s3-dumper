use std::{collections::VecDeque, path::PathBuf};

use futures_util::StreamExt;
use reqwest::{Client, Url};
use tokio::{
    fs::{create_dir_all, File},
    io::AsyncWriteExt,
    task::JoinSet,
};

const MAX_PARALLEL_DOWNLOADS: usize = 16;

pub struct Downloader {
    client: Client,
    save_path: PathBuf,
    url_root: Url,
    paths: VecDeque<String>,
    join_set: JoinSet<anyhow::Result<PathBuf>>,
}

impl Downloader {
    pub fn new(save_path: &str, url_root: &str, paths: VecDeque<String>) -> anyhow::Result<Self> {
        Ok(Self {
            client: Client::new(),
            save_path: save_path.into(),
            url_root: Url::parse(url_root)?,
            paths,
            join_set: JoinSet::new(),
        })
    }

    pub async fn wait_for_download(&mut self) -> anyhow::Result<Option<anyhow::Result<PathBuf>>> {
        // queue new tasks if possible
        while self.join_set.len() < MAX_PARALLEL_DOWNLOADS {
            if let Some(path) = self.paths.pop_front() {
                let mut url = self.url_root.clone();
                url.set_path(&path);
                let client = self.client.clone();

                let save_path = self.save_path.join(path);

                self.join_set.spawn(async move {
                    let resp = client.get(url).send().await?;
                    let mut stream = resp.bytes_stream();

                    // create parent path
                    let parent = save_path.parent().unwrap();
                    create_dir_all(parent).await?;

                    let mut file = File::create(&save_path).await?;
                    while let Some(chunk) = stream.next().await {
                        let mut chunk = chunk?;
                        file.write_all_buf(&mut chunk).await?;
                    }

                    Ok(save_path)
                });
            } else {
                break;
            }
        }

        // wait for the next task to finish
        if let Some(result) = self.join_set.join_next().await {
            Ok(Some(result?))
        } else {
            Ok(None)
        }
    }
}
