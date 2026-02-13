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
struct Language {
    name: String,
    color: String,
    bytes: i64, // FIXME: should be unsigned, but can't be bothered..
}

#[derive(IntoValue, IntoDict)]
struct Inputs {
    languages: Vec<Language>,
}

#[derive(Debug, Deserialize)]
struct LinguistLanguage {
    color: Option<String>,
}

pub async fn generate(
    github_username: String,
    skip_private: bool,
    skip_forks: bool,
    skip_archived: bool,
    skipped_languages: Vec<String>,
    num_languages: usize,
    output: PathBuf,
) -> color_eyre::Result<()> {
    if output.is_dir() {
        bail!("output must be a file. got `{}`", output.display());
    } else if output.extension().is_some_and(|ext| ext != "svg") {
        bail!("output must end in .svg. got `{}`", output.display());
    }

    let linguist_languages: HashMap<String, LinguistLanguage> = serde_saphyr::from_reader(
        Cursor::new(&mut include_bytes!("../../assets/languages.yml")),
    )?;

    for skipped_lang in &skipped_languages {
        if !linguist_languages
            .keys()
            .any(|k| k.to_lowercase() == skipped_lang.to_lowercase())
        {
            bail!("Language to skip `{}` is unknown", skipped_lang);
        }
    }

    let github_api_token = std::env::var("GITHUB_TOKEN")
        .wrap_err("GITHUB_TOKEN is needed to avoid rate-limitation")?;
    let crab = Arc::new(
        octocrab::Octocrab::builder()
            .personal_token(github_api_token)
            .build()?,
    );

    let pages_pb = ProgressBar::new_spinner().with_style(
        ProgressStyle::with_template("[{spinner}] {elapsed} | Fetching repos... {pos}").unwrap(),
    );
    pages_pb.enable_steady_tick(Duration::from_millis(100));

    let user = crab.users(github_username);

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

    eprintln!(
        "Fetched {} repos! Fetching language statistics for these...",
        repos.len()
    );

    let repo_pb = Arc::new(ProgressBar::new(repos.len() as u64).with_style(
        ProgressStyle::with_template("[{elapsed}] {wide_bar} {pos:>7}/{len:7} {msg}").unwrap(),
    ));

    let repo_language_hashmaps: Vec<_> = stream::iter(repos.into_iter())
        .map(|repo| {
            let repo_pb = &repo_pb;
            let crab = &crab;
            let skipped_languages = &skipped_languages;
            async move {
                repo_pb.set_message(repo.name);
                if skip_forks && repo.fork.unwrap() {
                    return None;
                }
                if skip_private && repo.private.unwrap() {
                    return None;
                }
                if skip_archived && repo.archived.unwrap() {
                    return None;
                }
                let repo_id = repo.id;

                // FIXME: dont' create an octocrab instance...
                let languages = crab
                    .repos_by_id(repo_id)
                    .list_languages()
                    .await
                    .unwrap() // FIXME:
                    .into_iter()
                    .filter(|(lang_name, _)| {
                        // FIXME: stop allocating strings.. can we precompute lowercases?
                        !skipped_languages.contains(&lang_name.to_lowercase())
                    });

                Some(languages)
            }
        })
        .buffer_unordered(10)
        .collect()
        .await;

    repo_pb.finish();

    let mut total_languages = Languages::new();
    for hashmap in repo_language_hashmaps.into_iter().flatten() {
        for (lang_name, bytes) in hashmap {
            total_languages
                .entry(lang_name)
                .and_modify(|total_bytes| *total_bytes += bytes)
                .or_insert(bytes);
        }
    }
    let mut total_languages_vec: Vec<_> = total_languages.into_iter().collect();
    total_languages_vec.sort_by_key(|(_, bytes)| !*bytes);

    if num_languages != 0 {
        let other_bytes = total_languages_vec
            .iter()
            .skip(num_languages)
            .fold(0, |acc, (_, bytes)| acc + bytes);
        total_languages_vec.truncate(num_languages);
        total_languages_vec.push(("Other".into(), other_bytes));
    }

    let languages = total_languages_vec
        .into_iter()
        .map(|(name, bytes)| {
            let color = linguist_languages
                .get(&name)
                .and_then(|l| l.color.clone())
                .unwrap_or_else(|| "#444".to_string());

            Language { name, bytes, color }
        })
        .collect();

    let output_pages = typst_bake::document!("languages.typ")
        .with_inputs(Inputs { languages })
        .to_svg()?;

    if output_pages.len() > 1 {
        bail!("Generated multiple pages, but only single-page output is supported");
    }

    if &output == "-" {
        println!("{}", output_pages[0]);
    } else {
        fs::write(&output, &output_pages[0])?;
        println!("Done! Outputted to `{}`", output.display());
    }

    Ok(())
}
