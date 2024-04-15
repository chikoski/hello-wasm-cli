mod bindings;

fn main() {
    println!("Hello, world!");
    bindings::hello_world();
    
    let greeting = bindings::greet("Rust");
    println!("{}", greeting);
}
