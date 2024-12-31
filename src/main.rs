use std::fs::{self, File};
use std::io::{Write, Result};
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    fs::create_dir_all("lyrics")?;

    let texts_dir = Path::new("texts");
    for entry in fs::read_dir(texts_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("txt") {
            process_file(&path)?;
        }
    }

    Ok(())
}

fn process_file(input_path: &Path) -> Result<()> {
    let content = fs::read_to_string(input_path)?;

    let file_stem = input_path.file_stem().unwrap().to_str().unwrap();
    let output_path = PathBuf::from("lyrics").join(format!("{}.tex", file_stem));

    let slides: Vec<_> = content.split("\n\n").collect();
    let mut output = File::create(output_path)?;

    write!(output, "{}", r#"\documentclass{beamer}
\usepackage[T1]{fontenc}
\usepackage[utf8]{inputenc}
\usepackage{xcolor}
\usetheme{default}
\setbeamertemplate{navigation symbols}{}
\setbeamertemplate{footline}{}
\setbeamercolor{background canvas}{bg=black}
\begin{document}
\color{white}
"#)?;

    for (i, slide) in slides.iter().enumerate() {
        write!(output, "\\begin{{frame}}\n\\centering\n")?;

        let lines: Vec<_> = slide.lines().collect();
        for line in &lines {
            if !line.is_empty() {
                writeln!(output, "{} \\\\", escape_latex(line))?;
            }
        }

        if i < slides.len() - 1 {
            let next_slide = slides[i + 1];
            if let Some(first_line) = next_slide.lines().next() {
                write!(output, "\n\\vfill\\small{{{}}}\n", escape_latex(first_line))?;
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
