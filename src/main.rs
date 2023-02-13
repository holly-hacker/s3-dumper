use anyhow::Context;
use clap::{Parser, Subcommand};
use client::S3Client;
use reqwest::Client;

use crate::client::S3Options;

mod client;
mod models;

lazy_static::lazy_static! {
    pub static ref CLIENT: Client = Client::new();
}

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    max_keys: Option<usize>,
    #[arg(long)]
    prefix: Option<String>,
    #[arg(long)]
    delimiter: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lists all files in an exposed bucket
    ListFiles { url: String },
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    if let Err(e) = async_main().await {
        println!("An error occured: {e:?}");
    }
}

async fn async_main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let options = S3Options {
        max_keys: cli.max_keys,
        prefix: cli.prefix.clone(),
        delimiter: cli.delimiter.clone(),
    };

    match cli.command {
        Commands::ListFiles { url } => {
            list_files(&url, options)
                .await
                .context("list files command")?;
        }
    }

    Ok(())
}

async fn list_files(url: &str, options: S3Options) -> anyhow::Result<()> {
    let client = S3Client::default();

    let mut continuation_token = None;

    loop {
        eprintln!("downloading with token {continuation_token:?}");
        let resp = client
            .fetch(url, &options, continuation_token.as_deref())
            .await
            .context("fetch page")?;
        continuation_token = resp.next_continuation_token.clone();
        eprintln!("found {} files", resp.contents.len());

        for item in resp.contents {
            println!("{}: {}", item.key, item.size);
        }

        if continuation_token.is_none() {
            break;
        }
    }
    eprintln!("finished downloading");

    Ok(())
}
