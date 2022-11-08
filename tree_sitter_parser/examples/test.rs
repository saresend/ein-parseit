
pub fn foo() {
    println!("in foo");
    bar()
}

fn bar() {
    for i in 0..10 {
        baz()
    }
}

fn baz() {
    println!("in baz"); 
}
