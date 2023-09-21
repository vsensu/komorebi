//! An empty application with default plugins.

use komorebi::prelude::*;

pub fn foo() {
    println!("foo");
}
pub fn foo1(_a: i32) {
    println!("foo1");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(foo)
        .add_system(foo1)
        .run();
}
