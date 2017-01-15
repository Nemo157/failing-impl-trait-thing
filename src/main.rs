extern crate futures;
extern crate tokio_core;
extern crate foo;

use tokio_core::reactor::Core;

fn main() {
    println!("{:?}", Core::new().unwrap().run(foo::Foo.bar()));
}
