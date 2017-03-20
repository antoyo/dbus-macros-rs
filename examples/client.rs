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

dbus_interface!("com.dbus.simple", interface Simple {
    fn hello() -> String;
});

dbus_interface!("com.dbus.test", interface Hello {
    fn hello() -> String;
    fn hello_with_name(name: &str) -> String;
    fn greeting(greeting: &str, name: &str) -> String;
    fn greeting_with_separator(greeting: &str, separator: &str, name: &str) -> String;
    fn greeting_with_separator_and_end(greeting: &str, separator: &str, name: &str, end: &str) -> String;
    fn to_string5(arg1: &str, arg2: &str, arg3: &str, arg4: &str, arg5: &str) -> String;
    fn int_to_string(int: i32) -> String;
    fn get_variable() -> i32;
    fn debug();
    fn debug1(arg1: &str);
    fn debug2(arg1: &str, arg2: &str);
    fn debug3(arg1: &str, arg2: &str, arg3: &str);
    fn debug4(arg1: &str, arg2: &str, arg3: &str, arg4: &str);
    fn debug5(arg1: &str, arg2: &str, arg3: &str, arg4: &str, arg5: &str);
});


fn main() {
    let hello = Hello::new("com.dbus.test", "/Hello", dbus::BusType::Session);

    match hello.hello() {
        Ok(string) => println!("{}", string),
        Err(error) => println!("Error calling DBus service: {}", error),
    }
    println!("{}", hello.hello_with_name("World").unwrap());
    println!("{}", hello.greeting("Hi", "Me").unwrap());
    println!("{}", hello.greeting_with_separator("Salut", " - ", "Toi").unwrap());
    println!("{}", hello.greeting_with_separator_and_end("Salut", " - ", "Toi", "?").unwrap());
    println!("{}", hello.to_string5("arg1 ", "arg2 ", "arg3 ", "arg4 ", "arg5").unwrap());
    let string: String = hello.int_to_string(42).unwrap();
    println!("{}", string);
    println!("{}", hello.get_variable().unwrap());
    hello.debug().unwrap();
    hello.debug1("arg1").unwrap();
    hello.debug2("arg1", "arg2").unwrap();
    hello.debug3("arg1", "arg2", "arg3").unwrap();
    hello.debug4("arg1", "arg2", "arg3", "arg4").unwrap();
    hello.debug5("arg1", "arg2", "arg3", "arg4", "arg5").unwrap();

    /*let simple = Simple::new("com.dbus.simple", "/Simple", dbus::BusType::Session);
    println!("{}", simple.hello().unwrap());*/
}
