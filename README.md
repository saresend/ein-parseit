
# Sample invocations 

Navigate to `tree_sitter_parser`, and then invoke the following: 


`cargo run -- --file examples/ast.rs`, 

**Note:** For some reason, it looks like relative file paths are resolved relative to the invocation `cwd`, *not* the project root. This 
means that the invocation directory actual matters, which is a pain. 
