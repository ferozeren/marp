use std::error::Error;
use std::fs;
use std::process::Command;

pub fn get_markdown(input_md_path: &str) -> std::io::Result<String> {
    let markdown_input: String = fs::read_to_string(input_md_path)?;
    Ok(markdown_input)
}

pub fn md_convert(output_html: String, output_pdf: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("weasyprint")
        .arg(&output_html)
        .arg(output_pdf)
        .output();
    std::thread::sleep(std::time::Duration::from_secs(2));

    if let Err(e) = output {
        fs::remove_file(&output_html)?;
        eprintln!(
            "weasyprint failed to convert! Ensure weasyprint is installed and present in the path!: {}",
            e
        );
    } else {
        // Remove temp HTML file
        // fs::remove_file(&output_html)?;
        eprintln!("PDF Generated Successfully: {}", output_pdf);
    }
    Ok(())
}
