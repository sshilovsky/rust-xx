use libc::{
    c_int,
    c_long,
    c_uchar,
    c_ulong,
    c_void,
};
use std;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{
    CString,
};
use std::mem::{
    uninitialized,
};
use std::ptr;
use xlib;

#[allow(unused_imports)]
use xlib::{ 
    XCloseDisplay,
    XDefaultScreenOfDisplay,
    XDefaultRootWindow,
    XFree,
    XGetWindowProperty, 
    XInternAtom, 
    XOpenDisplay, 
    XRootWindowOfScreen,
    XSelectInput,
};

pub use self::atoms::Atom;
pub use self::atoms::CommonAtom::*;
pub use self::atoms::PredefinedAtom::*;

use self::event::EventMask;
pub use self::event::EventMask::*;

mod atoms;
mod consts;
mod event;

/// X11 display connection.
pub struct Display {
    pub xlib_display: *mut xlib::Display,
    atoms: RefCell<HashMap<String, xlib::Atom>>,
}

#[derive(Clone,Copy)]
pub struct Window(xlib::Window);

impl Drop for Display {
    fn drop(&mut self) { unsafe {
        XCloseDisplay(self.xlib_display);
    }}
}

impl Display {
    fn intern_atom(&self, atom_name: String) -> xlib::Atom {
        {
            let map = self.atoms.borrow();

            if let Some(x) = map.get(&atom_name) {
                return *x
            }
        }
        // TODO thread-safety in borrow_mut call
        let mut map = self.atoms.borrow_mut();

        let value: xlib::Atom = unsafe { XInternAtom(self.xlib_display,
                                      CString::new(atom_name.clone()).unwrap().as_ptr() as *mut i8, 0) };
        map.insert(atom_name, value);
        value
    }

    pub fn open_default() -> Option<Display> { unsafe {
        let xlib_display = XOpenDisplay(ptr::null_mut());
        if xlib_display == ptr::null_mut() { return None }

        Some(Display {
            xlib_display: xlib_display,
            atoms: RefCell::new(HashMap::new()),
        })
    }}

    pub fn default_root_window(&self) -> Window {
        Window( unsafe { XDefaultRootWindow(self.xlib_display) } )
    }

    pub fn select_input(&self, window: Window, masks: &[EventMask]) {
        let mask: c_long = masks.iter().fold(0, |res, x| res | (*x as c_long));

        let result = unsafe { XSelectInput(self.xlib_display, window.0, mask) };
        println!("XSelectInput -> {}", result);
        // result is ignored as e.g. 1 (BadRequest) may not be fail
    }

    pub fn get_window_property<T:Atom+std::fmt::Debug+Sized>(&self, window: Window, property: T) -> Option<WindowProperty> {
        unsafe {
            let mut return_type: xlib::Atom = uninitialized();
            let mut return_format: c_int = uninitialized();
            let mut return_nitems: c_ulong = uninitialized();
            let mut return_bytes_after: c_ulong = uninitialized();
            let mut return_buffer: *mut c_uchar = uninitialized();

            let result = XGetWindowProperty(
                self.xlib_display,
                window.0,
                property.to_atom(self),
                0,
                1024 * 1024, // buffer size
                0,
                consts::AnyPropertyType,
                &mut return_type,
                &mut return_format,
                &mut return_nitems,
                &mut return_bytes_after,
                &mut return_buffer);

            match result {
                consts::Success     => (),
                consts::BadAlloc    => return None,
                consts::BadValue    => return None,
                consts::BadWindow   => return None,
                consts::BadAtom     => unreachable!(),
                _                   => unreachable!(),
            }

            if return_type == consts::None { return None };

            Some({
                WindowProperty {
                    data_type: return_type,
                    format: return_format as u8,
                    data: return_buffer,
                    size: return_nitems as usize,
                }
            })
        }
    }
}

pub struct WindowProperty {
    data_type: xlib::Atom,
    pub format: u8, // valid values are 8, 16 and 32
    data: *mut c_uchar,
    size: usize,
}

impl WindowProperty {
    pub fn as_string(&self) -> Option<String> {
        if self.format != 8 { return None; }
        let vec = unsafe { Vec::from_raw_parts(self.data, self.size, self.size) };
        String::from_utf8(vec).ok()
    }
}

impl Drop for WindowProperty {
    fn drop(&mut self) {
        unsafe {
            XFree(self.data as *mut c_void);
        }
    }
}
