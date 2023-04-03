use crate::xlib::{AsRaw, Display};
use x11::xlib_xcb::*;

pub struct XCBConnection(*mut xcb_connection_t);

pub enum EventQueueOwner {
    XLibOwnsEventQueue = 0,
    XCBOwnsEventQueue = 1,
}

pub fn get_xcb_connection(display: &Display) -> XCBConnection {
    unsafe { XCBConnection(XGetXCBConnection(display.as_raw())) }
}

pub fn set_event_queue_owner(display: &Display, owner: EventQueueOwner) {
    unsafe {
        let owner = match owner {
            EventQueueOwner::XLibOwnsEventQueue => XEventQueueOwner::XlibOwnsEventQueue,
            EventQueueOwner::XCBOwnsEventQueue => XEventQueueOwner::XCBOwnsEventQueue,
        };

        XSetEventQueueOwner(display.as_raw(), owner);
    }
}
