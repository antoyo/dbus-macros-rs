/*
 * Copyright (c) 2016 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

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

    fn greeting(&this, greeting: &str, name: &str) -> String {
        format!("{}, {}!", greeting, name)
    }

    fn greeting_with_separator(&this, greeting: &str, separator: &str, name: &str) -> String {
        format!("{}{}{}!", greeting, separator, name)
    }

    fn greeting_with_separator_and_end(&this, greeting: &str, separator: &str, name: &str, end: &str) -> String {
        format!("{}{}{}{}", greeting, separator, name, end)
    }

    fn to_string5(&this, arg1: &str, arg2: &str, arg3: &str, arg4: &str, arg5: &str) -> String {
        format!("{}{}{}{}{}", arg1, arg2, arg3, arg4, arg5)
    }

    fn int_to_string(&this, int: i32) -> String {
        int.to_string()
    }

    fn get_variable(&this) -> i32 {
        this.variable
    }

    fn debug(&this) {
        println!("Debug");
    }

    fn debug1(&this, arg1: &str) {
        println!("Debug: {}", arg1);
    }

    fn debug2(&this, arg1: &str, arg2: &str) {
        println!("Debug: {} {}", arg1, arg2);
    }

    fn debug3(&this, arg1: &str, arg2: &str, arg3: &str) {
        println!("Debug: {} {} {}", arg1, arg2, arg3);
    }

    fn debug4(&this, arg1: &str, arg2: &str, arg3: &str, arg4: &str) {
        println!("Debug: {} {} {} {}", arg1, arg2, arg3, arg4);
    }

    fn debug5(&this, arg1: &str, arg2: &str, arg3: &str, arg4: &str, arg5: &str) {
        println!("Debug: {} {} {} {} {}", arg1, arg2, arg3, arg4, arg5);
    }
});

fn main() {
    let variable = 24;

    let hello = Hello::new(variable);
    hello.run("com.dbus.test");

    /*let simple = Simple::new();
    simple.run("com.dbus.simple");*/
}
