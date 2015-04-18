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

impl x11::Atom for CustomAtom {}

use CustomAtom::*;

fn main() {
    let display = x11::Display::open_default().expect("Failed to open display");

    let root = display.default_root_window();

    let property = display.get_window_property(root, _NET_DESKTOP_NAMES);

    println!("{:?}", property.expect("no such property").as_string());

    display.select_input(root, &[x11::PropertyChangeMask]);

    loop {
        let event = display.next_event();
        println!("next_event() -> {:?}", event);
    };

}
