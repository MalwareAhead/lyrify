use std::fs::{self, File};
use std::io::{Write, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> Result<()> {
    // Create output directory
    fs::create_dir_all("lyrics")?;

    // Process all txt files
    let texts_dir = Path::new("texts");
    let mut successful_compilations = Vec::new();
    let mut failed_compilations = Vec::new();

    for entry in fs::read_dir(texts_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("txt") {
            let file_stem = path.file_stem().unwrap().to_str().unwrap();
            match process_and_compile_file(&path) {
                Ok(_) => successful_compilations.push(file_stem.to_string()),
                Err(e) => {
                    eprintln!("Error processing {}: {}", file_stem, e);
                    failed_compilations.push(file_stem.to_string());
                }
            }
        }
    }

    // Print processing summary
    println!("\nProcessing Summary:");
    if !successful_compilations.is_empty() {
        println!("Successfully processed and compiled: {}", successful_compilations.join(", "));
    }
    if !failed_compilations.is_empty() {
        println!("Failed to process or compile: {}", failed_compilations.join(", "));
    }

    Ok(())
}

fn process_and_compile_file(input_path: &Path) -> Result<()> {
    let file_stem = input_path.file_stem().unwrap().to_str().unwrap();
    let output_path = PathBuf::from("lyrics").join(format!("{}.tex", file_stem));

    // Generate TEX file
    generate_tex_file(input_path, &output_path)?;

    // Compile to PDF
    println!("Compiling {} to PDF...", file_stem);

    let output = Command::new("pdflatex")
        .current_dir("lyrics")
        .arg("-interaction=nonstopmode")
        .arg(&format!("{}.tex", file_stem))
        .output()?;

    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("PDF compilation failed: {}", String::from_utf8_lossy(&output.stderr))
        ));
    }

    // Clean up auxiliary files
    let extensions_to_remove = ["aux", "log", "nav", "out", "snm", "toc"];
    for ext in extensions_to_remove {
        let aux_file = PathBuf::from("lyrics").join(format!("{}.{}", file_stem, ext));
        if aux_file.exists() {
            fs::remove_file(aux_file)?;
        }
    }

    Ok(())
}

fn generate_tex_file(input_path: &Path, output_path: &PathBuf) -> Result<()> {
    let content = fs::read_to_string(input_path)?;
    let slides: Vec<_> = content.split("\n\n").collect();
    let mut output = File::create(output_path)?;

	write!(output, "{}", r#"\documentclass[aspectratio=169]{beamer}
\usepackage[T1]{fontenc}
\usepackage[utf8]{inputenc}
\usepackage{xcolor}
\usepackage[absolute,overlay]{textpos}
\usetheme{default}
\setbeamertemplate{navigation symbols}{}
\setbeamertemplate{footline}{}
\setbeamercolor{background canvas}{bg=black}
\renewcommand{\rmdefault}{phv} % Arial
\renewcommand{\sfdefault}{phv} % Arial
\setlength{\TPHorizModule}{\paperwidth}
\setlength{\TPVertModule}{\paperheight}
\begin{document}
\color{white}
"#)?;

    for (i, slide) in slides.iter().enumerate() {
        write!(output, "\\begin{{frame}}\n\\centering\n\\color{{white}}\n")?;

        let lines: Vec<_> = slide.lines().collect();
        for line in &lines {
            if !line.is_empty() {
                writeln!(output, "\\huge{{{}}} \\\\", escape_latex(line))?;
            }
        }

        if i < slides.len() - 1 {
            let next_slide = slides[i + 1];
            if let Some(first_line) = next_slide.lines().next() {
                write!(output, "\n\\begin{{textblock}}{{1.0}}(0.00001,0.8)\\color{{gray}}\\normalsize{{{}}}\\end{{textblock}}\n", escape_latex(first_line))?;
            }
        }

        writeln!(output, "\\end{{frame}}\n")?;
    }

    write!(output, "\\end{{document}}\n")?;

    Ok(())
}

fn escape_latex(text: &str) -> String {
    text.replace("&", "\\&")
        .replace("%", "\\%")
        .replace("$", "\\$")
        .replace("#", "\\#")
        .replace("_", "\\_")
        .replace("{", "\\{")
        .replace("}", "\\}")
        .replace("~", "\\textasciitilde{}")
        .replace("^", "\\textasciicircum{}")
        .replace("\\", "\\textbackslash{}")
}
