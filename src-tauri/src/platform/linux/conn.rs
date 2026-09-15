use x11rb::connection::Connection;
use x11rb::protocol::xproto::{Atom, ConnectionExt, Window};
use x11rb::rust_connection::RustConnection;

pub struct Display {
    pub conn: RustConnection,
    pub root: Window,
}

pub fn open() -> Option<Display> {
    let (conn, screen) = x11rb::connect(None).ok()?;
    let root = conn.setup().roots.get(screen)?.root;
    Some(Display { conn, root })
}

pub fn atom(display: &Display, name: &str) -> Option<Atom> {
    display
        .conn
        .intern_atom(false, name.as_bytes())
        .ok()?
        .reply()
        .ok()
        .map(|reply| reply.atom)
}
