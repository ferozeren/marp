# Markdown to PDF Converter

A simple Rust command-line tool to convert Markdown files to PDF using the [weasyprint](https://doc.courtbouillon.org/weasyprint/stable/index.html) library. This tool reads a Markdown file, converts it to HTML with custom CSS styling, and then generates a PDF using `weasyprint`.
## Prerequisites
- **Rust**: Ensure you have Rust installed. You can install it using [rustup](https://rustup.rs/).
- **weasyprint**: Required for the Rust-based conversion. Install it using one of the following methods:

### Installing weasyprint

#### Option 1: Using OS Packaging
- **Ubuntu/Debian**:
  ```bash
  sudo apt-get install weasyprint
  ```
- **Arch Linux**:
  ```bash
  sudo pacman -S python-weasyprint
  ```
- **macOS** (using Homebrew):
  ```bash
  brew install weasyprint
  ```
- **Windows**: Download and install `weasyprint` from [Github](https://github.com/Kozea/WeasyPrint/releases) or follow manual [instructions](https://doc.courtbouillon.org/weasyprint/stable/first_steps.html#windows).

#### Option 2: Using pip
```bash
pip install weasyprint
```

#### Option 3: Using uv
```bash
uv tool install weasyprint
uv tool update-shell
```

Ensure `weasyprint` is accessible in your system's PATH after installation. Verify by running:
```bash
weasyprint --version
```

## Installation
1. Clone or download this repository:
   ```bash
   git clone https://github.com/ferozeren/marp.git
   cd marp
   ```
2. Build the Rust project using Cargo:
   ```bash
   cargo build --release
   ```
3. The executable will be available in `target/release/`.

4. Install marp using Cargo (Your Rust's bin path):
   ```bash
   cargo install --path .
   ```
  
## Usage
Run the Rust tool from the command line, providing the input Markdown file and the desired output PDF file:
```bash
marp input.md output.pdf
```

### Example
```bash
marp Sample.md Output.pdf
```
This will:
1. Read `Sample.md`.
2. Convert it to HTML with custom styling.
3. Use `weasyprint` to generate `Ouput.pdf`.
4. Remove the temporary HTML file(By default).

```bash
# Heading Level 1
## Heading Level 2
### Heading Level 3

---

This is a **bold** word, this is *italic*, this is ***bold italic***, and this is ~~strikethrough~~.

Inline code example: `let x = 42;`

A [link to Rust](https://www.rust-lang.org/) and an image:

![Ferris the crab](https://www.rust-lang.org/static/images/ferris.gif)

---

## Lists

**Unordered list:**
- Item one
- Item two
  - Subitem A
  - Subitem B

**Ordered list:**
1. First
2. Second
   1. Nested first
   2. Nested second

---

## Blockquote
> This is a blockquote.
> It can span multiple lines.
>
> > Nested blockquote.

---

## Code Blocks

```rust
fn main() {
    println!("Hello, world!");
}
```
# PDF Output

<img width="1426" height="1024" alt="Output" src="https://github.com/user-attachments/assets/45360ede-9a7a-41a9-9c8b-1eb9ea4f392f" />

### Input/Output Requirements
- Input file must have a `.md` or `.markdown` extension.
- Output file must have a `.pdf` extension.

### Error Handling
- If `weasyprint` is not installed or not in the PATH, the tool will display an error message.
- Invalid file extensions or incorrect arguments will result in appropriate error messages.

## Custom Styling (Rust Tool)
The Rust tool applies the following CSS for PDF generation:
- Page size: A4
- Page margin: 15mm
- Font: DejaVu Sans
- Code blocks and inline code: Light gray background with rounded corners
- Headings: Dark blue color (#2c3e50)
- Line height: 1.6 for better readability

To modify the styling, edit the `custom_css` function in the source code.

## Dependencies
- **Rust Tool**:
  - Rust crates:
    - `std` (standard library)
    - `pulldown-cmark`: For Markdown parsing and HTML conversion
  - External tool:
    - `weasyprint`: For HTML-to-PDF conversion

## Building and Running (Rust Tool)
To build and run the Rust tool directly:
```bash
cargo run --release -- input.md output.pdf
```

## Troubleshooting
- **Rust Tool Errors**:
  - **Error: "weasyprint failed to convert!"**
    - Ensure `weasyprint` is installed and accessible in your PATH.
    - Verify installation with `weasyprint --version`.
  - **Error: "Invalid arguments"**
    - Check that you provided exactly two arguments: the input Markdown file and the output PDF file.
  - **Error: "File Extensions aren't provided properly!"**
    - Ensure the input file has a `.md` or `.markdown` extension and the output file has a `.pdf` extension.

## License
This project is licensed under the MIT License.
