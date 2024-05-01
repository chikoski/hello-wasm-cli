mod bindings;

use bindings::chikoski::name::name_provider::name;

fn main() {
    let n = name();
    println!("Hello, {}!", n);
}
