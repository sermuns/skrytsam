use color_eyre::eyre::{ContextCompat, bail};
use rust_embed::Embed;
use typst_as_lib::{TypstAsLibError, TypstEngine};
use typst_library::{
    diag::Warned,
    foundations::Dict,
    layout::{Page, PagedDocument},
};

const NOTO_SANS_REGULAR: &[u8] = include_bytes!("../assets/NotoSans-Regular.ttf");
const NOTO_SANS_BOLD: &[u8] = include_bytes!("../assets/NotoSans-Bold.ttf");
const MONASPACE_KRYPTON: &[u8] = include_bytes!("../assets/MonaspaceKrypton-Regular.otf");
const MONASPACE_KRYPTON_BOLD: &[u8] = include_bytes!("../assets/MonaspaceKrypton-Bold.otf");

const LANGUAGES_TEMPLATE: &str = include_str!("../typst/languages.typ");
const COMMON_LIB: &str = include_str!("../typst/lib.typ");

#[derive(Embed)]
#[folder = "typst/packages/"]
#[include = "*.typ"]
#[exclude = "**/tests/*"]
#[exclude = "**/gallery/*"]
struct TypstPackage;

impl TypstPackage {
    fn iter_path_contents()
}

pub fn compile_svg(input: Dict) -> color_eyre::Result<String> {
    let engine = TypstEngine::builder()
        .with_static_source_file_resolver([
            ("languages.typ", LANGUAGES_TEMPLATE),
            ("lib.typ", COMMON_LIB),
        ].iter().chain(TypstPackage::iter_path_contents()))
        .fonts([
            NOTO_SANS_REGULAR,
            NOTO_SANS_BOLD,
            MONASPACE_KRYPTON,
            MONASPACE_KRYPTON_BOLD,
        ])
        .build();

    let warned_document: Warned<Result<PagedDocument, TypstAsLibError>> =
        engine.compile_with_input("languages.typ", input);

    let warnings = warned_document.warnings;
    if !warnings.is_empty() {
        bail!("typst had warnings: {:#?}", warnings);
    }

    let output_pages: Vec<Page> = warned_document.output?.pages;
    if output_pages.len() > 1 {
        bail!("output document has multiple pages!")
    }

    let first_and_only_page = output_pages
        .first()
        .wrap_err("output document has no pages!")?;

    Ok(typst_svg::svg(first_and_only_page))
}
