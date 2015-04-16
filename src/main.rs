extern crate libc;
extern crate xlib;

mod x11;

use std::mem::{
    uninitialized,
};

#[derive(Debug)]
#[allow(non_camel_case_types)]
enum CustomAtom {
    _NET_CLIENT_LIST,
    _NET_DESKTOP_NAMES,
    _NET_SUPPORTED,
}

impl x11::Atom for CustomAtom {}

use CustomAtom::*;
use xlib::{XEvent, XNextEvent};

fn main() {
    let display = x11::Display::open_default().expect("Failed to open display");

    let screen = display.default_screen();
    let root = screen.root_window();

    let property = root.get_property(_NET_DESKTOP_NAMES);

    println!("{:?}", property.expect("no such property").as_string());

    root.select_input(&[x11::PropertyChangeMask]);

    unsafe {
        loop {
            let mut event: XEvent = uninitialized();
            let result = XNextEvent(display.xlib_display, &mut event);
            println!("XNextEvent -> {:?}", result);
        }
    };



}
