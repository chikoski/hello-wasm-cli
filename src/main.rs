mod bindings;

use bindings::chikoski::hello_world::chikoski_hello_world_greet::hello_world;
use bindings::chikoski::hello_world::chikoski_hello_world_greet::greet;

fn main() {
    println!("Hello, world!");
    println!("greet:hello_world() = {}", hello_world());
    
    let greeting = greet("Rust");
    println!("{}", greeting);
}
