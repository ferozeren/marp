pub fn custom_css() -> String {
    let page_size = "A4"; // or user input like "5in 8in"
    let page_margin = "12.5mm"; //  Narrow Margin
    let css: String = format!(
        r#"
<style>
    @page {{
        size: {};
        margin: {};
    }}

    body {{
        font-family: 'DejaVu Sans', sans-serif;
        font-size: 12px;
        line-height: 1.6;
        margin: 10px;
    }}

    h1, h2, h3 {{
        color: #2c3e50;
    }}

    code {{
        background: #f4f4f4;
        // font-family: 'DejaVu Sans Mono', monospace;
        font-family: 'JetBrains Mono', monospace;
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
