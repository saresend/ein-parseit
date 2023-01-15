use std::path::PathBuf;

fn main() {
    build_language("tree-sitter-rust")
}

fn build_language(language: &str) {
    let dir: PathBuf = ["vendor", language, "src"].iter().collect();

    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .file(dir.join("scanner.c"))
        .compile(language);
}
