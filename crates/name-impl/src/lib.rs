#[allow(warnings)]
mod bindings;

use bindings::exports::chikoski::name::name_provider::Guest;

struct Component;

impl Guest for Component {
    fn name() -> String {
        "chikoski".to_string()
    }
}

bindings::export!(Component with_types_in bindings);
