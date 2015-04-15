use libc::{
    c_int,
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
use std::rc::Rc;
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
pub use self::atoms::{
    CommonAtom,
    PredefinedAtom,
    ToAtom,
};

pub use self::atoms::CommonAtom::*;
pub use self::atoms::PredefinedAtom::*;

mod atoms;
mod consts;

struct DisplayStruct {
    xlib_display: *mut xlib::Display,
    atoms: RefCell<HashMap<String, Atom>>,
}

impl Drop for DisplayStruct {
    fn drop(&mut self) { unsafe {
        XCloseDisplay(self.xlib_display);
    }}
}

pub struct Display {
    unwrap: Rc<DisplayStruct>,
}

impl Display {
    pub fn clone(&self) -> Display {
        Display {
            unwrap: self.unwrap.clone(),
        }
    }

    fn intern_atom(&self, atom_name: String) -> Atom {
        {
            let map = self.unwrap.atoms.borrow();

            if let Some(x) = map.get(&atom_name) {
                return *x
            }
        }
        let mut map = self.unwrap.atoms.borrow_mut();

        unsafe {
            let value: Atom = XInternAtom(self.unwrap.xlib_display,
                                          CString::new(atom_name.clone()).unwrap().as_ptr() as *mut i8, 0);
            map.insert(atom_name, value);
            value
        }
    }

    pub fn get_atom<T:ToAtom+std::fmt::Debug>(&self, atom: T) -> Atom {
        atom.to_atom(self)
    }

    pub fn open_default() -> Option<Display> { unsafe {
        let xlib_display = XOpenDisplay(ptr::null_mut());
        if xlib_display == ptr::null_mut() { return None }

        Some(Display {
            unwrap: Rc::new(DisplayStruct {
                xlib_display: xlib_display,
                atoms: RefCell::new(HashMap::new()),
            }),
        })
    }}

    pub fn default_screen(&self) -> Screen { unsafe {
        let screen = XDefaultScreenOfDisplay(self.unwrap.xlib_display);
        assert!(screen != ptr::null_mut());

        Screen {
            display: self.clone(),
            screen: screen,
            root: Window {
                display: self.clone(),
                window: XRootWindowOfScreen(screen),
            },
        }
    }}

}

pub struct Screen {
    display: Display,
    screen: *mut xlib::Screen,
    pub root: Window,
}

pub struct Window {
    display: Display,
    window: xlib::Window,
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
                self.display.unwrap.xlib_display,
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

pub struct WindowProperty {
    pub data_type: Atom,
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
