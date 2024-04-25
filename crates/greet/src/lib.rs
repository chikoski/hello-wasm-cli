#[allow(warnings)]
mod bindings;

use bindings::chikoski::name::name_provider::name;
use bindings::exports::chikoski::hello::greet::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello() -> String {
        let n = name();
        Self::greet(n)
    }

    fn greet(name: String) -> String {
        format!("Hello, {}!", name)
    }
}

bindings::export!(Component with_types_in bindings);
