#![feature(libc)]

extern crate libc;
extern crate xlib;
extern crate xinerama;

mod x11;

fn main() {
    let display = x11::Display::open_default().expect("Failed to open display");

    println!("{:?} {:?}", display.atoms._NET_SUPPORTED, display.atoms._NET_CLIENT_LIST);

    let screen = display.default_screen();
    let root = screen.root;

    let property = root.get_property(display.atoms._NET_DESKTOP_NAMES);

    println!("{:?}", property);
    println!("{:?}", property.unwrap().as_string());
}
