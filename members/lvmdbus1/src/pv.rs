use crate::{LvmConn, LvmPath, Nodes};
use dbus::{
    self, arg,
    stdintf::org_freedesktop_dbus::{Introspectable, Properties},
    BusType, ConnPath, Connection,
};

pub struct PvConn {
    conn: Connection,
}

impl PvConn {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self { conn: Connection::get_private(BusType::System)? })
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = PvPath<'a>> {
        let path = self.conn.with_path("com.redhat.lvmdbus1", "/com/redhat/lvmdbus1/Pv", 1000);

        path.introspect()
            .map_err(|why| {
                eprintln!("{:?}", why);
                why
            })
            .ok()
            .into_iter()
            .map(|xml| serde_xml_rs::from_str::<Nodes>(xml.as_str()).unwrap())
            .flat_map(|nodes| nodes.nodes)
            .map(move |id| self.connect(&id.name))
    }
}

impl<'a> LvmConn<'a> for PvConn {
    type Item = PvPath<'a>;

    const DEST: &'static str = "com.redhat.lvmdbus1";
    const OBJECT: &'static str = "/com/redhat/lvmdbus1/Vg";

    fn conn(&self) -> &Connection { &self.conn }
}

pub struct PvPath<'a> {
    conn: ConnPath<'a, &'a Connection>,
}

impl<'a> LvmPath<'a> for PvPath<'a> {
    const PATH: &'static str = "com.redhat.lvmdbus1.Pv";

    fn conn<'b>(&'b self) -> &'b ConnPath<'a, &'a Connection> { &self.conn }

    fn from_path(conn: ConnPath<'a, &'a Connection>) -> Self { Self { conn } }
}
