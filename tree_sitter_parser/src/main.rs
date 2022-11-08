use tree_sitter::{Parser, Language};

use std::fs;

#[link(name = "tree-sitter-rust")]
extern "C" { fn tree_sitter_rust() -> Language; }

fn read_sample_file() -> String {
    fs::read_to_string("examples/ast.rs").unwrap()
}

pub fn invoke_tree_sitter() {
    let mut parser = Parser::new();

    let language = unsafe { tree_sitter_rust() };
    parser.set_language(language).unwrap();
    

    let source_code = read_sample_file();
    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();

    let mut walker = tree.walk();
    loop{
        println!("{}", walker.node().kind());
        if walker.goto_first_child() == false {
            break;
        }
    }

    println!("{}",root_node.start_position());



}



fn main() {
    invoke_tree_sitter();
}
