use tree_sitter::{Parser, Language};


extern "C" { fn tree_sitter_rust() -> Language; }

pub fn test_parse() {
    let mut parser = Parser::new();

    let language = unsafe { tree_sitter_rust() };
    parser.set_language(language).unwrap();
    

    let source_code = "fn test() {}";
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
