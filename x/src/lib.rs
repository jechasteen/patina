#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::os::raw::{c_uint};
use xcb::Connection;
use xcb::ffi::{
    xproto,
    base,
};

const XCB_MOVE: u16 = (xproto::XCB_CONFIG_WINDOW_X | xproto::XCB_CONFIG_WINDOW_Y) as u16;
const XCB_RESIZE: u16 = (xproto::XCB_CONFIG_WINDOW_WIDTH | xproto::XCB_CONFIG_WINDOW_HEIGHT) as u16;
const XCB_MOVE_RESIZE: u16 = XCB_MOVE | XCB_RESIZE;

struct Screen {
    handle: i32,
    width: u16,
    height: u16,
}

pub struct Instance {
    connection: Connection,
    screen: Screen,
}

impl Instance {
    pub fn new() -> Self {
        if let Ok((conn, scr)) = Connection::connect(None) {
            Self {
                connection: conn,
                screen: Screen { handle: scr, width: 0, height: 0 },
            }
        } else {
            panic!("Failed to initialize X instance! Check your DISPLAY variable.");
        }
        
    }

    pub fn setup(&mut self) -> std::io::Result<()> {
        let setup = self.connection.get_setup();
        let screen = setup.roots().nth(self.screen.handle as usize).unwrap();
        self.screen.width = screen.width_in_pixels();
        self.screen.height = screen.height_in_pixels();
        // init client tree
        let mask = xproto::XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY | xproto::XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT;
        unsafe {
            xproto::xcb_change_window_attributes(self.conn(), screen.root(), xproto::XCB_CW_EVENT_MASK, &mask);
            self.flush();
        }

        Ok(())
    }

    fn flush(&self) {
        unsafe {
            let res = base::xcb_flush(self.conn());
            if res > 0 {
                println!("xcb_flush() output go down the hole.");
            } else {
                panic!("xcb_flush() failed!");
            }
        }
    }

    fn conn(&self) -> *mut base::xcb_connection_t {
        self.connection.get_raw_conn()
    }

    pub fn run(&mut self) {
        loop {
            let event_ptr;
            let event: Option<base::xcb_generic_event_t>;
            unsafe { 
                event_ptr = base::xcb_poll_for_event(self.conn());
                if !event_ptr.is_null() {
                    event = Some(*event_ptr);
                } else {
                    event = None
                }
            };
            if let Some(e) = event {
                match e.response_type {
                    xproto::XCB_MAP_REQUEST => {
                        println!("XCB_MAP_REQUEST");
                        let map_event;
                        unsafe {
                            map_event = *std::mem::transmute::<*mut base::xcb_generic_event_t, *mut xproto::xcb_map_request_event_t>(event_ptr)
                        };
                        self.map_request(map_event);
                        self.flush();
                    },
                    xproto::XCB_DESTROY_NOTIFY => {
                        println!("XCB_DESTROY_NOTIFY")
                    },
                    _ => println!("Unimplemented: {:?}", e.response_type)
                }
            }
        }
    }

    fn map_request(&mut self, event: xproto::xcb_map_request_event_t) {
        println!("Map Event.");
        //create window
        unsafe {
            xproto::xcb_map_window(self.conn(), event.window);
            xproto::xcb_configure_window(self.conn(), event.window, xproto::XCB_CONFIG_WINDOW_BORDER_WIDTH as u16, &5);
            let frame: [c_uint; 4] = [10, 10, 200, 200];
            xproto::xcb_configure_window(self.conn(), event.window, XCB_MOVE_RESIZE, frame.as_ptr());
        }
        println!("Mapped window {}", event.window);
    }
}