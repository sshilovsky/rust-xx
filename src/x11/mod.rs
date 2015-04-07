use libc::{
    c_int,
    c_uchar,
    c_ulong,
};
use std::ffi::{
    CString,
};
use std::mem::{
    transmute,
    uninitialized,
};
use std::ptr;
use xlib;
use xlib::{ 
    XCloseDisplay,
    XDefaultScreenOfDisplay,
    XFree,
    XGetWindowProperty, 
    XInternAtom, 
    XOpenDisplay, 
    XRootWindowOfScreen
};

pub use xlib::Atom;

pub mod atoms;
pub mod consts;

#[allow(non_snake_case)]
pub struct Atoms {
    pub _NET_CLIENT_LIST: Atom,
    pub _NET_DESKTOP_NAMES: Atom,
    pub _NET_SUPPORTED: Atom,
}

pub struct Display {
    display: *mut xlib::Display,
    pub atoms: Atoms,
}

pub struct Screen {
    display: *mut xlib::Display,
    screen: *mut xlib::Screen,
    pub root: Window,
}

pub struct Window<'a> {
    display: &'a Display,
    window: xlib::Window,
}

#[derive(Debug)]
pub struct WindowProperty {
    pub data_type: Atom,
    pub format: u8, // valid values are 8, 16 and 32
    data: *mut c_uchar,
    size: usize,
}

impl Display {
    pub fn open_default() -> Option<Display> { unsafe {
        let display = XOpenDisplay(ptr::null_mut());
        if display == ptr::null_mut() { return None }

        Some(Display {
                display: display,
                atoms: Display::load_atoms(display),
        })
    }}

    unsafe fn load_atoms(display: *mut xlib::Display) -> Atoms {
        Atoms {
            _NET_CLIENT_LIST: XInternAtom(display, CString::new("_NET_CLIENT_LIST").unwrap().as_ptr() as *mut i8, 0),
            _NET_DESKTOP_NAMES: XInternAtom(display, CString::new("_NET_DESKTOP_NAMES").unwrap().as_ptr() as *mut i8, 0),
            _NET_SUPPORTED: XInternAtom(display, CString::new("_NET_SUPPORTED").unwrap().as_ptr() as *mut i8, 0),
        }
    }

    pub fn default_screen(&self) -> Screen { unsafe {
        let screen = XDefaultScreenOfDisplay(self.display);
        assert!(screen != ptr::null_mut());

        Screen{
            display: self.display,
            screen: screen,
            root: Window {
                display: self.display,
                window: XRootWindowOfScreen(screen),
            },
        }
    }}
}

impl Drop for Display {
    fn drop(&mut self) { unsafe {
        XCloseDisplay(self.display);
    }}
}

impl Window {
    pub fn get_property(&self, property: Atom) -> Option<WindowProperty> {
        unsafe {
            let mut return_type: Atom = uninitialized();
            let mut return_format: c_int = uninitialized();
            let mut return_nitems: c_ulong = uninitialized();
            let mut return_bytes_after: c_ulong = uninitialized();
            let mut return_buffer: *mut c_uchar = uninitialized();

            let result = XGetWindowProperty(
                self.display,
                self.window,
                property,
                0,
                1024 * 1024, // buffer size
                0,
                consts::AnyPropertyType,
                &mut return_type,
                &mut return_format,
                &mut return_nitems,
                &mut return_bytes_after,
                &mut return_buffer);

            if result != consts::Success { return None };

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

impl WindowProperty {
    pub fn as_string(&self) -> Option<String> {
        if self.data_type == atoms::STRING {
                if self.format != 8 { return None; }
                let vec = unsafe { Vec::from_raw_parts(self.data, self.size, self.size) };
                String::from_utf8(vec).ok()
            }
        else {
            None
        }
    }
}

impl Drop for WindowProperty {
    fn drop(&mut self) {
        unsafe {
            XFree(transmute(self.data));
        }
    }
}
