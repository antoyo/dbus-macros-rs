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

dbus_class!("com.dbus.simple", class Simple {
    fn hello(&this) -> String {
        "Hello!"
    }
});

dbus_class!("com.dbus.test", class Hello (variable: i32) {
    fn hello(&this) -> String {
        "Hello!"
    }

    fn hello_with_name(&this, name: &str) -> String {
        format!("Hello, {}!", name)
    }
});

fn main() {
    let variable = 24;

    let hello = Hello::new(variable);
    hello.run("com.dbus.test", dbus::BusType::Session, "/Hello");
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

dbus_interface!("com.dbus.test", interface Hello {
    fn hello() -> String;
    fn hello_with_name(name: &str) -> String;
});


fn main() {
    let hello = Hello::new("com.dbus.test", "/Hello", dbus::BusType::Session);

    match hello.hello() {
        Ok(string) => println!("{}", string),
        Err(error) => println!("Error calling DBus service: {}", error),
    }
    println!("{}", hello.hello_with_name("World").unwrap());
}
```

You can try a similar example (that tries more method calls on the server example) by running:

    cargo run --example client

Requirements
============

[dbus](https://github.com/diwic/dbus-rs) 0.5 or higher, but it's handled for you
by the cargo system.
