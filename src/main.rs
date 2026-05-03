use std::{error::Error, ffi::OsStr, fs, path::PathBuf};
mod css;
mod markdown;

use {
    css::custom_css,
    markdown::{get_markdown, md_convert},
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!(
            "Invalid argumensts\nExample: {} input.md output.pdf",
            args[0]
        );
        return Ok(());
    }

    let input_md: &str = &args[1];
    let output_pdf: &str = &args[2];

    let input_md_path: PathBuf = PathBuf::from(&input_md);
    let output_pdf_path: PathBuf = PathBuf::from(&output_pdf);

    if let Some(ext_md) = input_md_path.extension()
        && let Some(ext_pdf) = output_pdf_path.extension()
    {
        if ext_pdf != OsStr::new("pdf") {
            eprintln!("Output file doesn't have pdf extension!");
            return Ok(());
        }
        if ext_md != OsStr::new("md") && ext_md != OsStr::new("markdown") {
            eprintln!("Input file has not given any markdown extesnion i.e. md, markdown");
            return Ok(());
        }
    } else {
        eprintln!("File Extensions aren't provided properly!");
        return Ok(());
    }

    let output_html: String = output_pdf_path
        .with_extension("html")
        .to_string_lossy()
        .into_owned();

    let markdown_data: String = get_markdown(input_md)?;
    let mut options: pulldown_cmark::Options = pulldown_cmark::Options::empty();
    // options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    options.insert(pulldown_cmark::Options::all());
    let parser = pulldown_cmark::Parser::new_ext(&markdown_data, options);

    let mut html_data: String = String::new();
    pulldown_cmark::html::push_html(&mut html_data, parser);
    let css: String = custom_css();

    // let final_html = format!(
    //     "<html><head>{}</head><body>{}</body></html>",
    //     css, html_data
    // );

    let final_html = format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    {}
</head>
<body>{}</body>
</html>
"#,
        css, html_data
    );

    fs::write(&output_html, final_html)?;
    md_convert(output_html, output_pdf)?;

    Ok(())
}
