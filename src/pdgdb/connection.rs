use rusqlite::Connection;
use dotenv::dotenv;

pub fn connect() -> Result<Connection, Box<dyn std::error::Error>> {
    dotenv().ok();
    let pdgdb = std::env::var("PDGDB_PATH").unwrap_or_else(|_| "./pdg-2024-v0.1.3.sqlite".to_string());
    let conn = Connection::open(pdgdb)?;
    Ok(conn)
}