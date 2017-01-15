#![feature(conservative_impl_trait)]

extern crate futures;

use futures::{ future, Future };

pub struct Foo;

impl Foo {
    pub fn bar(self) -> impl Future<Item=(), Error=String> {
        future::err(()).map_err(|err| format!("error: {:?}", err))
    }
}
