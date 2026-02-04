use std::borrow::Cow;

use color_eyre::eyre::{ContextCompat, bail};
use rust_embed::Embed;
use typst::syntax::{FileId, Source, VirtualPath};
use typst_as_lib::{TypstAsLibError, TypstEngine, conversions::IntoSource};
use typst_library::{
    diag::Warned,
    foundations::Dict,
    layout::{Page, PagedDocument},
};

const NOTO_SANS_REGULAR: &[u8] = include_bytes!("../assets/NotoSans-Regular.ttf");
const NOTO_SANS_BOLD: &[u8] = include_bytes!("../assets/NotoSans-Bold.ttf");
const MONASPACE_KRYPTON: &[u8] = include_bytes!("../assets/MonaspaceKrypton-Regular.otf");
const MONASPACE_KRYPTON_BOLD: &[u8] = include_bytes!("../assets/MonaspaceKrypton-Bold.otf");

#[derive(Embed)]
#[folder = "typst/"]
#[exclude = "**/tests/*"]
#[exclude = "**/gallery/*"]
struct TypstSource;

impl TypstSource {
    fn iter_sources() -> impl Iterator<Item = Source> {
        TypstSource::iter().filter_map(|path| {
            dbg!(&path);
            let embedded_file = TypstSource::get(&path)?;

            let contents = match embedded_file.data {
                Cow::Borrowed(bytes) => std::str::from_utf8(bytes).ok()?.to_string(),
                Cow::Owned(bytes) => String::from_utf8(bytes).ok()?,
            };

            let file_id = FileId::new(None, VirtualPath::new(path.as_ref()));
            Some((file_id, contents).into_source())
        })
    }
}

pub fn compile_svg(input: Dict) -> color_eyre::Result<String> {
    let engine = TypstEngine::builder()
        .with_static_source_file_resolver(TypstSource::iter_sources())
        .fonts([
            NOTO_SANS_REGULAR,
            NOTO_SANS_BOLD,
            MONASPACE_KRYPTON,
            MONASPACE_KRYPTON_BOLD,
        ])
        .build();

    let warned_document: Warned<Result<PagedDocument, _>> =
        engine.compile_with_input("languages.typ", input);

    let warnings = warned_document.warnings;
    if !warnings.is_empty() {
        bail!("typst had warnings: {:#?}", warnings);
    }

    let output_pages = warned_document.output?.pages;
    if output_pages.len() > 1 {
        bail!("output document has multiple pages!")
    }

    let first_and_only_page = output_pages
        .first()
        .wrap_err("output document has no pages!")?;

    Ok(typst_svg::svg(first_and_only_page))
}
