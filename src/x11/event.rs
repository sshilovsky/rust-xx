#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use super::Window;
use libc::{c_int, c_ulong};
use xlib;

// Input Event Masks. Used as event-mask window attribute and as arguments
//   to Grab requests.  Not to be confused with event names.

#[derive(Clone,Copy,Debug)]
pub enum EventMask {
    KeyPressMask            = 0x0000_0001,
    KeyReleaseMask          = 0x0000_0002,
    ButtonPressMask         = 0x0000_0004,
    ButtonReleaseMask       = 0x0000_0008,
    EnterWindowMask         = 0x0000_0010,
    LeaveWindowMask         = 0x0000_0020,
    PointerMotionMask       = 0x0000_0040,
    PointerMotionHintMask   = 0x0000_0080,

    Button1MotionMask       = 0x0000_0100,
    Button2MotionMask       = 0x0000_0200,
    Button3MotionMask       = 0x0000_0400,
    Button4MotionMask       = 0x0000_0800,
    Button5MotionMask       = 0x0000_1000,
    ButtonMotionMask        = 0x0000_2000,
    KeymapStateMask         = 0x0000_4000,
    ExposureMask            = 0x0000_8000,

    VisibilityChangeMask    = 0x0001_0000,
    StructureNotifyMask     = 0x0002_0000,
    ResizeRedirectMask      = 0x0004_0000,
    SubstructureNotifyMask  = 0x0008_0000,
    SubstructureRedirectMask= 0x0010_0000,
    FocusChangeMask         = 0x0020_0000,
    PropertyChangeMask      = 0x0040_0000,
    ColormapChangeMask      = 0x0080_0000,

    OwnerGrabButtonMask     = 0x0100_0000,
}

pub struct Event {
    // type field is included into underlying enum
    pub serial: c_ulong,
    pub send_event: bool,
    // includes display field
    pub window: Window,

    pub detail: EventDetail,
}

pub enum EventDetail {
    Unknown(c_int),
    PropertyNotify { atom: xlib::Atom, time: xlib::Time, state: c_ulong },
}

const KeyPress: c_int		= 2;
const KeyRelease: c_int		= 3;
const ButtonPress: c_int		= 4;
const ButtonRelease: c_int		= 5;
const MotionNotify: c_int		= 6;
const EnterNotify: c_int		= 7;
const LeaveNotify: c_int		= 8;
const FocusIn: c_int		= 9;
const FocusOut: c_int		= 10;
const KeymapNotify: c_int		= 11;
const Expose: c_int			= 12;
const GraphicsExpose: c_int		= 13;
const NoExpose: c_int		= 14;
const VisibilityNotify: c_int	= 15;
const CreateNotify: c_int		= 16;
const DestroyNotify: c_int		= 17;
const UnmapNotify: c_int		= 18;
const MapNotify: c_int		= 19;
const MapRequest: c_int		= 20;
const ReparentNotify: c_int		= 21;
const ConfigureNotify: c_int	= 22;
const ConfigureRequest: c_int	= 23;
const GravityNotify: c_int		= 24;
const ResizeRequest: c_int		= 25;
const CirculateNotify: c_int	= 26;
const CirculateRequest: c_int	= 27;
const PropertyNotify: c_int		= 28;
const SelectionClear: c_int		= 29;
const SelectionRequest: c_int	= 30;
const SelectionNotify: c_int	= 31;
const ColormapNotify: c_int		= 32;
const ClientMessage: c_int		= 33;
const MappingNotify: c_int		= 34;
const GenericEvent: c_int		= 35;
// #define LASTEvent		36
