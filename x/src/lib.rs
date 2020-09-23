#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

//use std::os::raw::{c_uint};
use xcb;
use xcb::Connection;
use xcb::ffi::{
    xproto,
    base,
};
use std::fs::{File, remove_file};
use std::path::Path;

const LOCK_FILE: &str = "/tmp/patina.lock";

const XCB_MOVE: u16 = (xproto::XCB_CONFIG_WINDOW_X | xproto::XCB_CONFIG_WINDOW_Y) as u16;
const XCB_RESIZE: u16 = (xproto::XCB_CONFIG_WINDOW_WIDTH | xproto::XCB_CONFIG_WINDOW_HEIGHT) as u16;
const XCB_MOVE_RESIZE: u16 = XCB_MOVE | XCB_RESIZE;

struct Screen {
    handle: *mut xproto::xcb_screen_t,
    width: u16,
    height: u16,
}

impl Screen {
    fn data(&self) -> &xproto::xcb_screen_t {
        unsafe {
            if !self.handle.is_null() {
                &*self.handle
            } else {
                panic!("Attempted to derefence null Screen struct!")
            }
        }
    }
}

struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32
}

impl Rect {
    pub fn as_ptr(&self) -> *const u32 {
        [self.x, self.y, self.width, self.height].as_ptr()
    }
}

struct Client {
    id: u32
}

pub struct Patina {
    connection: Connection,
    screen: Screen,
    clients: Vec<Client>
}

impl Drop for Patina {
    fn drop(&mut self) {
        match remove_file(LOCK_FILE) {
            Ok(()) => println!("Lock file deleted."),
            Err(e) => println!("Failed to delete lock file!\n{}", e)
        }
    }
}

impl Patina {
    pub fn new() -> Self {
        if Path::new(LOCK_FILE).exists() {
            panic!("Another instance of patina is already running!");
        }
        File::create(LOCK_FILE).expect("Failed to create lock file!");

        if let Ok((conn, _scr)) = Connection::connect(None) {
            let mut inst = Patina {
                connection: conn,
                screen: Screen { 
                    handle: std::ptr::null_mut(),
                    width: 0,
                    height: 0
                },
                clients: Vec::<Client>::new()
            };
            unsafe {
                inst.screen.handle = xproto::xcb_setup_roots_iterator(base::xcb_get_setup(inst.connection.get_raw_conn())).data;
            }
            inst
        } else {
            panic!("Failed to initialize X instance! Check your DISPLAY variable.");
        }
    }

    pub fn setup(&mut self) -> std::io::Result<()> {
        self.screen.width = self.screen.data().width_in_pixels;
        self.screen.height = self.screen.data().height_in_pixels;
        // init client tree
        let mask = xproto::XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY | xproto::XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT;
        unsafe {
            xproto::xcb_change_window_attributes(
                self.conn(), 
                (*self.screen.handle).root,
                xproto::XCB_CW_EVENT_MASK,
                &mask
            );
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
                            map_event = *std::mem::transmute::<*mut base::xcb_generic_event_t, *mut xproto::xcb_map_request_event_t>(event_ptr);
                            self.map_request(map_event);
                        }
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

    unsafe fn move_resize(conn: *mut base::xcb_connection_t, win: xproto::xcb_window_t, rect: Rect) {
        xproto::xcb_configure_window(conn, win, XCB_MOVE_RESIZE, rect.as_ptr());
    }

    fn get_id(&self) -> u32 {
        unsafe {
            base::xcb_generate_id(self.conn())
        }
    }

    unsafe fn map_request(&mut self, event: xproto::xcb_map_request_event_t) {
        xproto::xcb_map_window(self.conn(), event.window);

        xproto::xcb_configure_window(self.conn(), event.window, xproto::XCB_CONFIG_WINDOW_BORDER_WIDTH as u16, &5);
        Patina::move_resize(self.conn(), event.window, Rect { x: 10, y: 10, width: 500, height: 500 });
        println!("Mapped window {}", event.window);
        self.flush();
    }
}