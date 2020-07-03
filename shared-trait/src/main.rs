use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    static ref g: Global = Global::new();
}

struct Global {
    map: HashMap<i32, Arc<dyn Foo>>,
    // map: Arc<dyn Foo>,
}

impl Global {
    fn new() -> Global {
        Global{
            map: HashMap::new(),
            // map: Arc::new(Bar::new()),
        }
    }
}

trait Foo: Send + Sync {
}

struct Bar {

}

impl Bar {
    fn new() -> Bar {
        Bar{}
    }
}

impl Foo for Bar {}

fn main() {
}