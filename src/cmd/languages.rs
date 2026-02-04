use color_eyre::eyre::{WrapErr, bail};
use futures::{StreamExt, stream};
use indicatif::{ProgressBar, ProgressStyle};
use octocrab::models::{Repository, repos::Languages};
use serde::Deserialize;
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

    let linguist_languages: HashMap<String, LinguistLanguage> = serde_yaml_ng::from_reader(
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
    let octocrab = Arc::new(
        octocrab::Octocrab::builder()
            .personal_token(github_api_token)
            .build()?,
    );

    let user = octocrab.users(github_username);

    // TODO: make this concurrent..
    let mut repos: Vec<Repository> = Vec::new();
    for page_num in 1..u32::MAX {
        let mut page = user.repos().page(page_num).send().await?;
        let is_last_page = page.next.is_none();
        repos.append(&mut page.items);
        if is_last_page {
            break;
        }
    }

    let pb = Arc::new(
        ProgressBar::new(repos.len() as u64).with_style(
            ProgressStyle::with_template("[{elapsed_precise}] {wide_bar} {pos:>7}/{len:7} {msg}")
                .unwrap(),
        ),
    );

    let repo_language_hashmaps: Vec<_> = stream::iter(repos.into_iter())
        .map(|repo| {
            let pb = pb.clone();
            let octocrab = octocrab.clone();
            let skipped_languages = &skipped_languages;
            async move {
                pb.set_message(repo.name);
                if skip_forks && repo.fork.unwrap() {
                    return None;
                }
                if skip_private && repo.private.unwrap() {
                    return None;
                }
                let repo_id = repo.id;

                // FIXME: dont' create an octocrab instance...
                let languages = octocrab
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
    pb.finish();
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

    fs::write(&output, &output_pages[0])?;
    Ok(())
}
