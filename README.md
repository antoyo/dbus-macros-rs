----

No longer maintained or developed. Dropped in favor of [zbus](https://crates.io/crates/dbus)
----

D-Bus macros for Rust
=====================

Dealing with D-Bus in your code can be a bit tedious. These macros makes the
task simpler. They are inspired by [Vala's awesome D-Bus support](https://chebizarro.gitbooks.io/the-vala-tutorial/content/d-bus-integration.html).

Examples
========

Server
------

This example serves a bunch of methods on an object

```rust
extern crate dbus;
#[macro_use]
extern crate dbus_macros;

use dbus::{Connection, BusType};
use std::rc::Rc;

dbus_class!("com.dbus.test", class Hello (variable: i32) {
    fn hello(&this) -> String {
        "Hello!"
    }

    fn hello_with_name(&this, name: &str) -> String {
        format!("Hello, {}!", name)
    }

    fn get_variable(&this) -> i32 {
        this.variable
    }
});

fn main() {
    let variable = 24;
    let session_connection = Connection::get_private(BusType::Session).unwrap();
    let hello = Hello::new(variable);
    hello.run("com.dbus.test", &session_connection, "/Hello");
}
```

You can try a similar example (which has more methods) by running:

    cargo run --example server

Client
------

This example opens a connection to the server example above and calls its methods.

```rust
extern crate dbus;
#[macro_use]
extern crate dbus_macros;

use dbus::{Connection, BusType};
use std::rc::Rc;

dbus_interface!("com.dbus.test", interface Hello {
    fn hello() -> String;
    fn hello_with_name(name: &str) -> String;
    fn get_variable() -> i32;
});

fn main() {
    let session_connection = std::rc::Rc::new(dbus::Connection::get_private(dbus::BusType::Session).unwrap());
    let hello = Hello::new("com.dbus.test", "/Hello", session_connection);

    match hello.hello() {
        Ok(string) => println!("{}", string),
        Err(error) => println!("Error calling DBus service: {}", error),
    }
    println!("{}", hello.hello_with_name("World").unwrap());
    println!("{}", hello.get_variable().unwrap());
}
```

You can try a similar example (that tries more method calls on the server example) by running:

    cargo run --example client

Requirements
============

[dbus](https://github.com/diwic/dbus-rs) 0.5 or higher, but it's handled for you
by the cargo system.
