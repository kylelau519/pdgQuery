use rusqlite::Connection;
use dotenv::{dotenv, from_path};

pub fn connect() -> Result<Connection, Box<dyn std::error::Error>> {
    let cargo_dir = env!("CARGO_MANIFEST_DIR");
    let dotenv_path = format!("{}/.env", cargo_dir);
    from_path(&dotenv_path).ok();

    let pdgdb = std::env::var("PDGDB_PATH").unwrap();
    let conn = Connection::open(pdgdb)?;
    Ok(conn)
}

#[cfg(test)]
mod test{

    use super::*;
    #[test]
    fn test_db_env_path()-> Result<(), Box<dyn std::error::Error>>{
        let cargo_dir = env!("CARGO_MANIFEST_DIR");
        let dotenv_path = format!("{}/.env", cargo_dir);
        from_path(&dotenv_path).ok();
        dbg!(std::env::vars());
        let pdgpath = std::env::var("PDGDB_PATH").unwrap();
        assert_eq!(pdgpath, "/Users/kylelau519/Programming/pdgQuery/pdg-2024-v0.1.3.sqlite");
        Ok(())
    }
}