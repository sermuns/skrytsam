use color_eyre::eyre::{self, WrapErr, bail};
use futures::TryStreamExt;
use futures::{StreamExt, stream};
use indicatif::{ProgressBar, ProgressStyle};
use octocrab::models::{Repository, repos::Languages};
use serde::Deserialize;
use std::time::Duration;
use std::{collections::HashMap, fs, io::Cursor, path::PathBuf, sync::Arc};
use typst_bake::{IntoDict, IntoValue};

#[derive(IntoValue, IntoDict)]
struct Inputs {
    // languages: Vec<Language>,
}

pub async fn generate(
    github_username: String,
    skip_private: bool,
    skip_forks: bool,
    skip_archived: bool,
    output: PathBuf,
) -> color_eyre::Result<()> {
    let github_api_token = std::env::var("GITHUB_TOKEN")
        .wrap_err("GITHUB_TOKEN is needed to avoid rate-limitation")?;
    let crab = Arc::new(
        octocrab::Octocrab::builder()
            .personal_token(github_api_token)
            .build()?,
    );
    let user = crab.users(github_username);

    let pages_pb = ProgressBar::new_spinner().with_style(
        ProgressStyle::with_template("[{spinner}] {elapsed} | Fetching repos... {pos}").unwrap(),
    );
    pages_pb.enable_steady_tick(Duration::from_millis(100));

    let first_page = user.repos().per_page(100).send().await?;

    let repos: Vec<Repository> = if let Some(last_page_uri) = first_page.last {
        let last_page_query = last_page_uri.query().unwrap();
        let last_page_num = last_page_query
            .split('&')
            .find_map(|param| {
                let mut split = param.split('=');
                let key = split.next()?;
                let value = split.next()?;
                (key == "page").then_some(value.parse::<u32>().ok()?)
            })
            .unwrap();
        stream::iter(2..=last_page_num)
            .map(|page_num| {
                let user = &user;
                let pages_pb = &pages_pb;
                async move {
                    let page = user.repos().per_page(100).page(page_num).send().await?;
                    pages_pb.inc(page.items.len() as u64);
                    eyre::Ok(page)
                }
            })
            .buffer_unordered(10)
            .try_fold(first_page.items, |mut acc, page| async move {
                acc.extend(page.items);
                eyre::Ok(acc)
            })
            .await?
    } else {
        first_page.items
    };

    pages_pb.finish();

    println!(
        "Fetched {} repos! Generating contribution statistics...",
        repos.len()
    );

    let repo_pb = Arc::new(ProgressBar::new(repos.len() as u64).with_style(
        ProgressStyle::with_template("[{elapsed}] {wide_bar} {pos:>7}/{len:7} {msg}").unwrap(),
    ));

    repo_pb.finish();

    let output_pages = typst_bake::document!("contributions.typ")
        // .with_inputs(todo!())
        .to_svg()?;

    if output_pages.len() > 1 {
        bail!("Generated multiple pages, but only single-page output is supported");
    }

    fs::write(&output, &output_pages[0])?;

    println!("Done! Outputted to `{}`", output.display());
    Ok(())
}
