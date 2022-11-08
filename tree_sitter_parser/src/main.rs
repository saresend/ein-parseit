use tree_sitter::{Parser, Language, Tree, TreeCursor};
use std::fs;


#[link(name = "tree-sitter-rust")]
extern "C" { fn tree_sitter_rust() -> Language; }

fn read_sample_file() -> String {
    fs::read_to_string("examples/test.rs").unwrap()
}

pub fn invoke_tree_sitter() {
    let mut parser = Parser::new();

    let language = unsafe { tree_sitter_rust() };
    parser.set_language(language).unwrap();
    

    let source_code = read_sample_file();

    let base_tree = parser.parse(source_code, None).unwrap();
    let base_tree = base_tree.walk();

    print_serialized_parse_tree(0, &base_tree);
    }

pub fn print_serialized_parse_tree<'a>(offset: usize, current_cursor: &'a TreeCursor) {
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



fn main() {
    invoke_tree_sitter();
}
