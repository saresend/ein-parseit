use tree_sitter::{Language, TreeCursor};
use clap;
use std::fs;

/*
 * Key initialization code for extern based c language for tree sitter. 
 */

#[link(name = "tree-sitter-rust")]
extern "C" { fn tree_sitter_rust() -> Language; }



pub fn invoke_tree_sitter<'a>(input_file: &'a std::path::PathBuf) {

    let mut parser = tree_sitter::Parser::new();
    let language = unsafe { tree_sitter_rust() };
    parser.set_language(language).unwrap();
    
    let source_code = fs::read_to_string(input_file).unwrap();

    let base_tree = parser.parse(source_code, None).unwrap();
    let base_tree = base_tree.walk();

    print_serialized_parse_tree(0, &base_tree);
    }


/// Baseline for basic debugging and grokking the resulting parse tree of 
/// different files for rust
///
/// as an example, when run against `examples/test.rs`
///
///{Node source_file (1, 0) - (15, 0)}
///  {Node function_item (6, 0) - (10, 1)}
///  {Node identifier (6, 3) - (6, 6)}
///    {Node parameters (6, 6) - (6, 8)}
///      {Node ) (6, 7) - (6, 8)}
///    {Node block (6, 9) - (10, 1)}
///      {Node expression_statement (7, 4) - (9, 5)}
///      {Node } (10, 0) - (10, 1)}
///  {Node function_item (12, 0) - (14, 1)}
///    {Node identifier (12, 3) - (12, 6)}
///    {Node parameters (12, 6) - (12, 8)}
///      {Node ) (12, 7) - (12, 8)}
///    {Node block (12, 9) - (14, 1)}
///      {Node expression_statement (13, 4) - (13, 23)}
///        {Node ; (13, 22) - (13, 23)}
///      {Node } (14, 0) - (14, 1)}
///

fn print_serialized_parse_tree<'a>(offset: usize, current_cursor: &'a TreeCursor) {
    print!("{: <1$}", "", offset);
    println!("{:?}", current_cursor.node());
    let mut next = current_cursor.clone();

    if !next.goto_first_child() {
        return;
    }

    while next.goto_next_sibling() {
        print_serialized_parse_tree(offset + 2, &mut next);
    }

}

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
#[derive(Debug)]
struct Parseit {
    #[arg(short, long, value_name = "FILE")]
    file: std::path::PathBuf,
}
use clap::Parser;
fn invoke_clap_cli() -> Parseit {
    Parseit::parse()    
}



fn main() {
    let cli_input = invoke_clap_cli();
    let input_file = cli_input.file;
    invoke_tree_sitter(&input_file);    
}
