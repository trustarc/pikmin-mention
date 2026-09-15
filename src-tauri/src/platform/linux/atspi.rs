use zbus::blocking::connection::Builder;
use zbus::blocking::{Connection, Proxy};
use zbus::names::OwnedBusName;
use zbus::zvariant::{ObjectPath, OwnedObjectPath};

const REGISTRY: &str = "org.a11y.atspi.Registry";
const ROOT: &str = "/org/a11y/atspi/accessible/root";
const ACCESSIBLE: &str = "org.a11y.atspi.Accessible";
const TEXT: &str = "org.a11y.atspi.Text";

const ADDRESS_BAR_ROLES: &[&str] = &["entry", "combo box"];

const MAX_DEPTH: usize = 10;
const MAX_NODES: usize = 1500;

type Reference = (String, OwnedObjectPath);

fn bus() -> Option<Connection> {
    let session = Connection::session().ok()?;

    let address: String = session
        .call_method(
            Some("org.a11y.Bus"),
            "/org/a11y/bus",
            Some("org.a11y.Bus"),
            "GetAddress",
            &(),
        )
        .ok()?
        .body()
        .deserialize()
        .ok()?;

    Builder::address(address.as_str()).ok()?.build().ok()
}

fn proxy<'a>(
    conn: &'a Connection,
    name: &str,
    path: &ObjectPath<'a>,
    interface: &str,
) -> Option<Proxy<'a>> {
    Proxy::new(
        conn,
        name.to_string(),
        path.to_owned(),
        interface.to_string(),
    )
    .ok()
}

fn children(conn: &Connection, target: &Reference) -> Vec<Reference> {
    let Ok(path) = ObjectPath::try_from(target.1.as_str()) else {
        return Vec::new();
    };
    let Some(accessible) = proxy(conn, &target.0, &path, ACCESSIBLE) else {
        return Vec::new();
    };

    accessible
        .call::<_, _, Vec<Reference>>("GetChildren", &())
        .unwrap_or_default()
}

fn role_of(conn: &Connection, target: &Reference) -> Option<String> {
    let path = ObjectPath::try_from(target.1.as_str()).ok()?;
    let accessible = proxy(conn, &target.0, &path, ACCESSIBLE)?;
    accessible.call::<_, _, String>("GetRoleName", &()).ok()
}

fn text_of(conn: &Connection, target: &Reference) -> Option<String> {
    let path = ObjectPath::try_from(target.1.as_str()).ok()?;
    let text = proxy(conn, &target.0, &path, TEXT)?;
    text.call::<_, _, String>("GetText", &(0i32, -1i32)).ok()
}

fn looks_like_url(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty() && !trimmed.contains(' ') && !trimmed.contains('@') && trimmed.contains('.')
}

fn find_url(
    conn: &Connection,
    target: &Reference,
    depth: usize,
    budget: &mut usize,
) -> Option<String> {
    if depth > MAX_DEPTH || *budget == 0 {
        return None;
    }
    *budget -= 1;

    let is_address_bar =
        role_of(conn, target).is_some_and(|role| ADDRESS_BAR_ROLES.contains(&role.as_str()));

    if is_address_bar {
        if let Some(found) = text_of(conn, target).filter(|value| looks_like_url(value)) {
            return Some(found);
        }
    }

    for child in children(conn, target) {
        if let Some(found) = find_url(conn, &child, depth + 1, budget) {
            return Some(found);
        }
    }

    None
}

fn connection_pid(session: &Connection, name: &str) -> Option<u32> {
    session
        .call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "GetConnectionUnixProcessID",
            &(name,),
        )
        .ok()?
        .body()
        .deserialize()
        .ok()
}

pub fn browser_url_via_a11y(pid: i32) -> Option<String> {
    if pid <= 0 {
        return None;
    }

    let conn = bus()?;
    let root_path = ObjectPath::try_from(ROOT).ok()?;
    let registry = proxy(&conn, REGISTRY, &root_path, ACCESSIBLE)?;

    let apps: Vec<Reference> = registry.call("GetChildren", &()).ok()?;

    for app in apps {
        let Ok(name) = OwnedBusName::try_from(app.0.as_str()) else {
            continue;
        };

        if connection_pid(&conn, name.as_str()) != Some(pid as u32) {
            continue;
        }

        let mut budget = MAX_NODES;
        if let Some(found) = find_url(&conn, &app, 0, &mut budget) {
            return Some(found);
        }
    }

    None
}
