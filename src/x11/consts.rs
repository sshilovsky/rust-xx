#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use xlib::Atom;
use libc::{
    c_int,
};

pub const AnyPropertyType: Atom = 0;
pub const None: Atom = 0;

// ERROR CODES
pub const Success : c_int = 		   0;	// everything's okay
pub const BadRequest : c_int = 	   1;	// bad request code
pub const BadValue : c_int = 	   2;	// int parameter out of range
pub const BadWindow : c_int = 	   3;	// parameter not a Window
pub const BadPixmap : c_int = 	   4;	// parameter not a Pixmap
pub const BadAtom : c_int = 		   5;	// parameter not an Atom
pub const BadCursor : c_int = 	   6;	// parameter not a Cursor
pub const BadFont : c_int = 		   7;	// parameter not a Font
pub const BadMatch : c_int = 	   8;	// parameter mismatch
pub const BadDrawable : c_int = 	   9;	// parameter not a Pixmap or Window
pub const BadAccess : c_int = 	  10;	// depending on context:
				//  - key/button already grabbed
				//  - attempt to free an illegal 
				//    cmap entry 
				// - attempt to store into a read-only 
				//    color map entry.
 				// - attempt to modify the access control
				//    list from other than the local host.

pub const BadAlloc : c_int = 	  11;	// insufficient resources
pub const BadColor : c_int = 	  12;	// no such colormap
pub const BadGC : c_int = 		  13;	// parameter not a GC
pub const BadIDChoice : c_int = 	  14;	// choice not in range or already used
pub const BadName : c_int = 		  15;	// font or color name doesn't exist
pub const BadLength : c_int = 	  16;	// Request length incorrect
pub const BadImplementation : c_int =  17;	// server is defective

pub const FirstExtensionError : c_int = 	128;
pub const LastExtensionError : c_int = 	255;
