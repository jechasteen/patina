#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use xcb;
use xcb::ffi::{base, xproto};

enum GCMask {
    Function = 1,
    PlaneMask = 2,
    Foreground = 4,
    Background = 8,
    LineWidth = 16,
    LineStyle = 32,
    CapStyle = 64,
    JoinStyle = 128,
    FillStyle = 256,
    FillRule = 512,
    Tile = 1024,
    Stipple = 2048,
    TileStippleOriginX = 4096,
    TileStippleOriginY = 8192,
    Font = 16384,
    SubwindowMode = 32768,
    GraphicsExposures = 65536,
    ClipOriginX = 131072,
    ClipOriginY = 262144,
    ClipMask = 524288,
    DashOffset = 1048576,
    DashList = 2097152,
    ArcMode = 4194304,
}

pub struct Screen {
    pub handle: *mut xproto::xcb_screen_t,
    pub width: u16,
    pub height: u16,
}

impl Screen {
    pub fn data(&self) -> &xproto::xcb_screen_t {
        unsafe {
            if !self.handle.is_null() {
                &*self.handle
            } else {
                panic!("Attempted to derefence null Screen struct!")
            }
        }
    }
}

pub struct XConnection {
    pub connection: xcb::Connection,
    pub screen: Screen,
}

impl XConnection {
    pub fn conn(&self) -> *mut base::xcb_connection_t {
        self.connection.get_raw_conn()
    }
}

pub struct Color {
    r: u16,
    g: u16,
    b: u16,
}

pub fn create_colormap(conn: &XConnection, window: xproto::xcb_window_t, colors: Vec<Color>) {
    unsafe {
        let map = base::xcb_generate_id(conn.conn());
        for color in colors.into_iter() {
            xproto::xcb_alloc_color(conn.conn(), map, color.r, color.g, color.b);
        }

        xproto::xcb_create_colormap(
            conn.conn(),
            xproto::XCB_COLORMAP_ALLOC_NONE as u8,
            map,
            window,
            conn.screen.data().root_visual,
        );
    }
}

pub struct Surface {
    ctx: u32,
}

impl Surface {
    fn create(conn: &XConnection, drawable: u32, mask: GCMask, colors: Vec<Color>) -> Self {
        let ctx;
        unsafe {
            ctx = base::xcb_generate_id(conn.conn());
        }
        create_colormap(conn, drawable, colors);
        unsafe {
            xproto::xcb_create_gc(
                conn.conn(),
                ctx,
                drawable,
                mask as u32,
                &conn.screen.data().black_pixel,
            );
        }
        Surface { ctx }
    }
}
