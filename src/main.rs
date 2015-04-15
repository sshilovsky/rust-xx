extern crate libc;
extern crate xlib;

mod x11;

#[derive(Debug)]
#[allow(non_camel_case_types)]
enum CustomAtom {
    _NET_CLIENT_LIST,
    _NET_DESKTOP_NAMES,
    _NET_SUPPORTED,
}

impl x11::ToAtom for CustomAtom {}

use CustomAtom::*;

fn main() {
    let display = x11::Display::open_default().expect("Failed to open display");

    println!("{:?} {:?}", display.get_atom(x11::PIXMAP), display.get_atom(_NET_DESKTOP_NAMES));

    let screen = display.default_screen();
    let root = screen.root_window();

    let property = root.get_property(display.get_atom(_NET_DESKTOP_NAMES));

    println!("{:?}", property.expect("no such property").as_string());
}
