#![allow(non_upper_case_globals)]

use xlib::Atom;
use libc::{
    c_int,
};

pub static AnyPropertyType: Atom = 0;
pub static None: Atom = 0;
pub static Success: c_int = 0;
