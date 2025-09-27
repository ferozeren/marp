use std::{error::Error, ffi::OsStr, fs, path::PathBuf, process::Command};

fn get_makdown(input_md: String) -> Result<String, Box<dyn Error>> {
    let markdown_input: String = fs::read_to_string(input_md)?;
    Ok(markdown_input)
}

fn custom_css() -> String {
    let page_size = "A4"; // or user input like "5in 8in"
    let page_margin = "15mm"; // or user input like "15mm"

    let css: String = format!(
        r#"
<style>
    @page {{
        size: {};
        margin: {};
    }} 

    body {{
        font-family: 'DejaVu Sans', sans-serif;
        line-height: 1.6;
        margin: 10px;
    }}

    h1, h2, h3 {{
        color: #2c3e50;
    }}

    code {{
        background: #f4f4f4;
        padding: 2px 4px;
        border-radius: 4px;
    }}

    pre {{
        background: #f4f4f4;
        padding: 10px;
        border-radius: 6px;
    }}

    ul {{
        margin-left: 10px;
    }}
</style>
"#,
        page_size, page_margin
    );

    css
}

fn md_convert(output_html: String, output_pdf: String) -> Result<(), Box<dyn Error>> {
    // let status: std::process::ExitStatus = Command::new("weasyprint")
    //     .arg(&output_html)
    //     .arg(&output_pdf)
    //     .status()?;

    let output= Command::new("weasyprint")
        .arg(&output_html)
        .arg(&output_pdf)
        .output();
    
    if let Err(e) = output {
        fs::remove_file(&output_html)?;
        eprintln!(
            "weasyprint failed to convert! Ensure weasyprint is installed and present in the path!: {}",e
        );

    } else {

        // Remove temp HTML file
        fs::remove_file(&output_html)?;
        eprintln!("PDF Generated Successfully: {}", output_pdf);

    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args: Vec<String> = std::env::args().collect();
    let _ = args.iter_mut().map(|string| string.trim());

    if args.len() != 3 {
        eprintln!(
            "Invalid argumensts\nExample: {} input.md output.pdf",
            args[0]
        );
        return Ok(());
    }

    let input_md: String = args[1].clone();
    let output_pdf: String = args[2].clone();

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

    // println!("{:?}", output_html);

    let markdown_data: String = get_makdown(input_md)?;
    let mut options: pulldown_cmark::Options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    let parser = pulldown_cmark::Parser::new_ext(&markdown_data, options);

    let mut html_data: String = String::new();
    pulldown_cmark::html::push_html(&mut html_data, parser);

    let css: String = custom_css();

    let final_html = format!(
        "<html><head>{}</head><body>{}</body></html>",
        css, html_data
    );

    fs::write(&output_html, final_html)?;
    md_convert(output_html, output_pdf)?;

    Ok(())
}
