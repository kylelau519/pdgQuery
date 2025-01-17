use rusqlite::Connection;

pub fn connect() -> Result<Connection, Box<dyn std::error::Error>> {
    let pdgdb = "pdg-2024-v0.1.3.sqlite";
    let conn = Connection::open(pdgdb)?;
    Ok(conn)
}