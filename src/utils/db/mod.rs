use rusqlite::{Connection, Error, NO_PARAMS};

struct Server {
    uuid: u32,
    discord_uuid: u64,
}

pub fn migration() -> Result<(), Error> {
    println!("test")
}
